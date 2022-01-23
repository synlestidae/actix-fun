use actix;
use crate::Status;
use crate::TxMessage;
use crate::redis_client::CmdMsg;
use redis;
use serde_json;

pub struct RedisFormatter {
    uri: String,
    result_recipient: actix::Recipient<TxMessage<RedisResultMsg>>
}


pub struct RedisResultMsg {
    pub result: redis::RedisResult<()>
}

impl actix::Message for RedisResultMsg {
    type Result = ();
}

impl actix::Message for RedisFormatter {
    type Result=();
}

impl actix::Actor for RedisFormatter {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<Status>> for RedisFormatter {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<Status>, _ctx: &mut Self::Context) -> Self::Result {
        let mut client = redis::Client::open(self.uri.clone()).unwrap(); // TODO

        let result = redis::Cmd::new()
            .arg("SET")
            .arg(format!("statuses:{}:{}", msg.msg.account_id, msg.msg.name))
            .arg(serde_json::to_string(&msg.msg).unwrap())
            .query(&mut client);

        let result_msg = msg.map(RedisResultMsg { result });

        self.result_recipient.do_send(result_msg).unwrap();
    }
}
