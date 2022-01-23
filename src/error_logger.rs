use actix;
use crate::redis_client::RedisResultMsg;
use log::error;
use crate::TxMessage;
use crate::kafka_parser::ParseError;

pub struct ErrorLogger {
}

impl actix::Actor for ErrorLogger {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<RedisResultMsg>> for ErrorLogger {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<RedisResultMsg>, _ctx: &mut Self::Context) -> Self::Result {
        if let Err(err) = msg.msg.result {
            error!("{}: Redis error: {}", msg.id, err);
        } else {
            info!("{}: Successfully wrote status to redis", msg.id);
        }
    }
}

impl actix::Handler<TxMessage<ParseError>> for ErrorLogger {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<ParseError>, _ctx: &mut Self::Context) -> Self::Result {
        error!("{}: Error parsing kafka payload: {:?}", msg.id, msg.msg);
    }
}
