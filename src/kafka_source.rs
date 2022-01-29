use actix;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
//use kafka::consumer::Message;
use crate::TxMessage;
use log::info;
use std::thread::JoinHandle;

pub struct KafkaSource {
    _thread: JoinHandle<()>,
}

pub struct KafkaMessage {
    pub topic: String,
    pub offset: i64,
    pub key: Vec<u8>,
    pub body: Vec<u8>,
}

impl actix::Message for KafkaMessage {
    type Result = ();
}

// Thanks to https://github.com/kafka-rust/kafka-rust/blob/master/examples/console-consumer.rs

pub struct Config {
    brokers: Vec<String>,
    group: String,
    topics: Vec<String>,
    offset_storage: GroupOffsetStorage,
    fallback_offset: FetchOffset,
}

impl Config {
    pub fn new(brokers: Vec<String>, group: String, topics: Vec<String>) -> Self {
        Self {
            brokers,
            group,
            topics,
            offset_storage: GroupOffsetStorage::Kafka,
            fallback_offset: FetchOffset::Earliest,
        }
    }
}

impl KafkaSource {
    pub fn new(cfg: &Config, recipients: Vec<actix::Recipient<TxMessage<KafkaMessage>>>) -> Self {
        let mut c = {
            let mut cb = Consumer::from_hosts(cfg.brokers.clone())
                .with_group(cfg.group.clone())
                .with_fallback_offset(cfg.fallback_offset)
                .with_fetch_max_wait_time(std::time::Duration::from_secs(1))
                .with_fetch_min_bytes(1_000)
                .with_fetch_max_bytes_per_partition(100_000)
                .with_retry_max_bytes_limit(1_000_000)
                .with_offset_storage(cfg.offset_storage)
                .with_client_id("actix-fun-console-consumer".into());

            for topic in &cfg.topics {
                cb = cb.with_topic(topic.to_owned());
            }
            cb.create().unwrap()
        };

        let thread = std::thread::spawn(move || {
            info!("Started thread");
            loop {
                for ms in c.poll().unwrap().iter() {
                    for r in recipients.iter() {
                        for m in ms.messages() {
                            let km = KafkaMessage {
                                topic: ms.topic().to_owned(),
                                offset: m.offset,
                                key: m.key.iter().cloned().collect(),
                                body: m.value.iter().cloned().collect(),
                            };
                            let txm = TxMessage::new(km);
                            info!("{}: New message", txm.id);
                            r.do_send(txm).unwrap();
                        }
                    }
                }
            }
        });

        Self { _thread: thread }
    }
}

impl actix::Actor for KafkaSource {
    type Context = actix::Context<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // maybe kill the thread here?
    }
}
