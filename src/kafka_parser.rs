use crate::kafka_source::KafkaMessage;
use crate::Status;
use crate::TxMessage;
use actix;
use serde_json;
use serde_json::error::Error as SerdeErr;
use std::string::FromUtf8Error;

pub struct KafkaParser {
    status_recipients: Vec<actix::Recipient<TxMessage<Status>>>,
    error_recipient: actix::Recipient<TxMessage<ParseError>>,
}

impl KafkaParser {
    pub fn new(
        status_recipients: Vec<actix::Recipient<TxMessage<Status>>>,
        error_recipient: actix::Recipient<TxMessage<ParseError>>,
    ) -> Self {
        Self {
            status_recipients,
            error_recipient,
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
        info!("{}: Parsing Kafka message", msg.id);

        // use serde to parse fields
        let status_bytes: Vec<u8> = msg.msg.body.clone();
        let status_string = match String::from_utf8(status_bytes) {
            Ok(s) => s,
            Err(err) => {
                self.error_recipient
                    .do_send(msg.map(ParseError::Utf8Err(err)))
                    .unwrap(); // TODO
                return;
            }
        };
        let status = match serde_json::from_str(&status_string) {
            Ok(s) => s,
            Err(err) => {
                self.error_recipient
                    .do_send(msg.map(ParseError::SerdeErr(err)))
                    .unwrap(); // TODO
                return;
            }
        };

        info!("{}: Successfully parsed message", msg.id);

        let next_msg = msg.map(status);

        for recipient in self.status_recipients.iter() {
            recipient.do_send(next_msg.clone()).unwrap() // TODO
        }
    }
}
