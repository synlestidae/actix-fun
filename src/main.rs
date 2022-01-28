extern crate actix;
extern crate actix_rt;
extern crate async_std;
#[macro_use]
extern crate rocket;
extern crate serde;
extern crate uuid;
extern crate kafka;
extern crate chrono;
extern crate redis;
extern crate log;
extern crate env_logger;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
extern crate reqwest;

use actix::*;
use rocket::response::content::Json;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

mod kafka_source;
mod redis_client;
mod kafka_parser;
mod alert_analyser;
mod webhook_caller;
mod error_logger;

pub struct TxMessage<T> {
    pub msg: T,
    pub id: String
}

impl<T: Clone> Clone for TxMessage<T> {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            id: self.id.to_owned()
        }
    }
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

#[derive(Serialize, Deserialize, Clone)]
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

#[actix_rt::main]
async fn main() {
    env_logger::init();

    let cfg = kafka_source::Config::new(
        vec!["localhost:9092".to_owned()],
        "test_group".to_owned(),
        vec!["status_test".to_owned()]
    );
    let redis_uri = format!("redis://localhost");
    let error_logger_addr = (error_logger::ErrorLogger {}).start();

    let redis_client = redis_client::RedisClient::new(&redis_uri, error_logger_addr.clone().recipient());
    let redis_client_addr = redis_client.start();

    let webhook_caller = webhook_caller::WebhookCaller { 
        url: String::from("https://webhook.site/059609d2-ddf3-4290-87b9-e125a1317869")
    };
    let webhook_caller_addr = webhook_caller.start();

    let alert_analyser = alert_analyser::AlertAnalyser::new(
        vec![webhook_caller_addr.recipient()]
    );
    let alert_analyser_addr = alert_analyser.start();

    let kafka_parser = kafka_parser::KafkaParser::new(
        vec![
            redis_client_addr.recipient(),
            alert_analyser_addr.recipient()
        ],
        error_logger_addr.recipient()
    );
    let kafka_source = kafka_source::KafkaSource::new(
        &cfg,
        vec![kafka_parser.start().recipient()]
    );

    kafka_source.start();

    info!("Main thread started");

    loop {
        async_std::task::sleep(std::time::Duration::new(60 * 60 * 24, 0)).await; // sleep for a day idk
    }
}
