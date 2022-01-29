use crate::alert_analyser::StateChange;
use crate::TxMessage;
use actix_rt;
use log::{error, info};

pub struct WebhookCaller {
    pub url: String,
}

impl actix::Actor for WebhookCaller {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<StateChange>> for WebhookCaller {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<StateChange>, _ctx: &mut Self::Context) -> Self::Result {
        let url = self.url.clone();

        info!(
            "{}: Received state change. Spawning an upload task.",
            msg.id
        );

        // we do not want to block the handle() method and
        // we aren't using async-await right now.
        // luckily actix_rt has spawn_blocking. an improvement would be to 
        // spawn a future, or change this to an async handler

        actix_rt::task::spawn_blocking(move || {
            info!("{}: Uploading to {}", msg.id, url);

            let client = reqwest::blocking::Client::new();
            let res = client
                .post(&url)
                .body(serde_json::to_string(&msg.msg).unwrap())
                .send();

            if let Err(err) = res {
                error!("{}: Error posting to webhook: {}", msg.id, err);
            } else {
                info!("{}: Posting to the webhook was a success", msg.id);
            }
        });
    }
}
