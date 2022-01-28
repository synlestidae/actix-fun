use crate::alert_analyser::StateChange;
use crate::TxMessage;
use actix_rt;
use log::error;

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
        actix_rt::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post(&url)
                .body(serde_json::to_string(&msg.msg).unwrap())
                .send();

            if let Err(err) = res {
                error!("Error posting to webhook: {}", err);
            }
        });
    }
}
