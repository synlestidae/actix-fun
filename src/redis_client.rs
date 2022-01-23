use redis;
use actix;
use crate::TxMessage;
use redis::RedisResult;

pub struct CmdMsg {
    pub cmd: redis::Cmd
}

impl actix::Message for CmdMsg {
    type Result = ();
}

pub struct RedisClient {
    uri: String,
    result_recipient: actix::Recipient<TxMessage<RedisResultMsg>>
}

impl RedisClient {
    fn new(uri: &str, result_recipient: actix::Recipient<TxMessage<RedisResultMsg>>) -> Self {
        Self {
            uri: uri.to_owned(),
            result_recipient
        }
    }
}

impl actix::Actor for RedisClient {
    type Context = actix::Context<Self>;
}

pub struct RedisResultMsg {
    pub result: RedisResult<()>
}

impl actix::Message for RedisResultMsg {
    type Result = ();
}

impl actix::Handler<TxMessage<CmdMsg>> for RedisClient {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<CmdMsg>, _ctx: &mut Self::Context) -> Self::Result {
        // TODO check the account_id and name keys
        
        let mut client = redis::Client::open(self.uri.clone()).unwrap(); // TODO
        let result = msg.msg.cmd.query::<()>(&mut client);
        let result_msg = msg.map(RedisResultMsg { result });
        self.result_recipient.do_send(result_msg).unwrap();
    }
}
