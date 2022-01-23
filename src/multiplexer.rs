use actix;

pub struct Multiplexer<M> 
    where M: actix::Message + Send + Sync, M::Result: Send + Sync {
    recipients: Vec<actix::Recipient<M>>
}

#[derive(Clone)]
pub struct MultiplexMsg<M: actix::Message + Send + Sync>(M);

impl<M: actix::Message + Send + Sync> actix::Message for MultiplexMsg<M> {
    type Result = Result<(), String>;
}

impl<M> Multiplexer<M> where M: actix::Message + Send + Sync, M::Result: Send + Sync {
    pub fn new(recipients: Vec<actix::Recipient<M>>) -> Self {
        Self {
            recipients
        }
    }
}

impl<M: 'static> actix::Actor for Multiplexer<M> where M: actix::Message + Send + Sync, M::Result: Send + Sync {
    type Context = actix::Context<Self>;
}

impl<M: Clone> actix::Handler<MultiplexMsg<M>> for Multiplexer<M> where M: actix::Message + Send + Sync + 'static, M::Result: Send + Sync {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: MultiplexMsg<M>, _ctx: &mut Self::Context) -> Self::Result {
        let errors = self.recipients.iter().map(|r| if r.do_send(msg.0.clone()).is_ok() { 
            1
        } else {
            0
        }).sum();

        match errors {
            0 => Ok(()),
            err => Err(format!("{} send error(s)", err))
        }
    }
}
