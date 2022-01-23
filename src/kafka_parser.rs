use actix;
use crate::kafka_source::KafkaMessage;
use serde_json;
use serde_json::error::Error as SerdeErr;
use crate::Status;
use std::string::FromUtf8Error;
use crate::TxMessage;

pub struct KafkaParser {
    status_recipient: actix::Recipient<TxMessage<Status>>,
    error_recipient: actix::Recipient<TxMessage<ParseError>>
}

impl KafkaParser {
    pub fn new(
        status_recipient: actix::Recipient<TxMessage<Status>>,
        error_recipient: actix::Recipient<TxMessage<ParseError>>
    ) -> Self {
        Self {
            status_recipient,
            error_recipient
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Utf8Err(FromUtf8Error),
    SerdeErr(SerdeErr),
}

impl actix::Message for ParseError {
    type Result = ();
}

impl actix::Actor for KafkaParser {
    type Context = actix::Context<Self>;
}

impl actix::Handler<TxMessage<KafkaMessage>> for KafkaParser {
    type Result = ();

    fn handle(&mut self, msg: TxMessage<KafkaMessage>, _ctx: &mut Self::Context) -> Self::Result {
        // use serde to parse fields
        let status_bytes: Vec<u8> = msg.msg.body.clone();
        let status_string = match String::from_utf8(status_bytes) {
            Ok(s) => s,
            Err(err) => {
                self.error_recipient.do_send(msg.map(ParseError::Utf8Err(err))).unwrap(); // TODO
                return;
            }
        };
        let status = match serde_json::from_str(&status_string) {
            Ok(s) => s,
            Err(err) => {
                self.error_recipient.do_send(msg.map(ParseError::SerdeErr(err))).unwrap(); // TODO
                return;
            }
        };
        self.status_recipient.do_send(msg.map(status)).unwrap() // TODO
    }
}
