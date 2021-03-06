use crate::Status;
use crate::TxMessage;
use actix;
use log::info;
use serde::Serialize;
use std::collections::HashMap;

/// Consumes Status messages and produces StateChange messages 
/// if the status field is different from the previous message.
pub struct AlertAnalyser {
    state_change_recipients: Vec<actix::Recipient<TxMessage<StateChange>>>,
    statuses: HashMap<String, Status>,
}

impl AlertAnalyser {
    pub fn new(state_change_recipients: Vec<actix::Recipient<TxMessage<StateChange>>>) -> Self {
        Self {
            state_change_recipients,
            statuses: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize)]
/// Represents a status change. The first status seen by the system
/// is also considered a status change.
pub struct StateChange {
    pub previous_status: Option<Status>,
    pub current_status: Status,
}

impl actix::Message for StateChange {
    type Result = ();
}

impl actix::Actor for AlertAnalyser {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<Status>> for AlertAnalyser {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<Status>, _ctx: &mut Self::Context) -> Self::Result {
        info!("{}: Handling a new status message", msg.id);
        // compute what kind of change

        let change = match self.statuses.get(&msg.msg.name) {
            None => InternalState::New,
            Some(status) => {
                if status.status != msg.msg.status {
                    InternalState::Changed {
                        previous: status.clone(),
                    }
                } else {
                    InternalState::Unchanged
                }
            }
        };

        self.statuses
            .insert(msg.msg.name.to_owned(), msg.msg.clone());

        let current_status = msg.msg.clone();
        let id = msg.id.clone();
        let name = msg.msg.name.clone();

        // the next block either exits this method 
        // or builds the status_change msg for the next
        // actors

        let status_change_msg = msg.map(match change {
            InternalState::New => {
                info!("{}: First status message for `{}`", id, name);
                StateChange {
                    previous_status: None,
                    current_status,
                }
            }
            InternalState::Changed { previous } => {
                info!("{}: Status has changed for `{}`", id, name);
                StateChange {
                    previous_status: Some(previous.clone()),
                    current_status,
                }
            }
            InternalState::Unchanged => {
                info!("{}: No status change for `{}`", id, name);
                return;
            }
        });

        for recipient in self.state_change_recipients.iter() {
            if let Err(err) = recipient.do_send(status_change_msg.clone()) {
                error!(
                    "{}: There was an error sending status change message to {:?}: {}",
                    id, recipient, err,
                );
            }
        }
    }
}

enum InternalState {
    New,
    Changed { previous: Status },
    Unchanged,
}
