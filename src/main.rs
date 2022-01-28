extern crate actix;
extern crate actix_rt;
extern crate async_std;
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate env_logger;
extern crate kafka;
extern crate lettre;
extern crate lettre_email;
extern crate log;
extern crate native_tls;
extern crate redis;
extern crate reqwest;
extern crate serde;
extern crate uuid;

use actix::*;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

mod alert_analyser;
mod error_logger;
mod kafka_parser;
mod kafka_source;
mod redis_client;
mod webhook_caller;

pub struct TxMessage<T> {
    pub msg: T,
    pub id: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub brokers: Vec<String>,
    pub group: String,
    pub topic: String,
    pub webhook_url: String,
    pub redis_uri: String,
}

impl<T: Clone> Clone for TxMessage<T> {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            id: self.id.to_owned(),
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
            msg,
        }
    }

    pub fn map<U>(self, other: U) -> TxMessage<U> {
        TxMessage {
            msg: other,
            id: self.id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Status {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub description: String,
    pub account_id: String,
}

impl actix::Message for Status {
    type Result = ();
}

#[actix_rt::main]
async fn main() {
    env_logger::init();

    info!("Loading config from config.json...");

    let config_string = std::fs::read_to_string("config.json").expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_string).expect("Unable to parse config file");

    let cfg = kafka_source::Config::new(config.brokers, config.group, vec![config.topic]);
    let error_logger_addr = (error_logger::ErrorLogger {}).start();

    let redis_client =
        redis_client::RedisClient::new(&config.redis_uri, error_logger_addr.clone().recipient());
    let redis_client_addr = redis_client.start();

    let webhook_caller = webhook_caller::WebhookCaller {
        url: config.webhook_url,
    };
    let webhook_caller_addr = webhook_caller.start();

    let alert_analyser = alert_analyser::AlertAnalyser::new(vec![webhook_caller_addr.recipient()]);
    let alert_analyser_addr = alert_analyser.start();

    let kafka_parser = kafka_parser::KafkaParser::new(
        vec![
            redis_client_addr.recipient(),
            alert_analyser_addr.recipient(),
        ],
        error_logger_addr.recipient(),
    );
    let kafka_source = kafka_source::KafkaSource::new(&cfg, vec![kafka_parser.start().recipient()]);

    kafka_source.start();

    info!("Main thread started");

    loop {
        async_std::task::sleep(std::time::Duration::new(60 * 60 * 24, 0)).await;
        // sleep for a day idk
    }
}
