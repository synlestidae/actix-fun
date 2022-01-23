extern crate actix;
#[macro_use]
extern crate rocket;
extern crate serde;
extern crate uuid;
extern crate kafka;
extern crate chrono;
extern crate redis;

use actix::*;
use rocket::response::content::Json;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

mod kafka_source;
mod multiplexer;
mod redis_formatter;
mod redis_client;
mod kafka_parser;

pub struct TxMessage<T> {
    pub msg: T,
    pub id: String
}

impl<T: actix::Message> actix::Message for TxMessage<T> {
    type Result = T::Result;
}

impl<T> TxMessage<T> {
    pub fn new(msg: T) -> Self {
        TxMessage {
            id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
            msg
        }
    }

    pub fn map<U>(self, other: U) -> TxMessage<U>{
        TxMessage {
            msg: other,
            id: self.id
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub description: String,
    pub account_id: String
}

impl actix::Message for Status {
    type Result = ();
}

fn main() {
}
