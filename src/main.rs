extern crate actix;
#[macro_use]
extern crate rocket;
extern crate serde;

use actix::*;
use rocket::response::content::Json;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Transaction {
    id: Option<String>,
    from: String,
    to: String,
    amount: i64,
    description: String
}

pub struct NewTransaction(Transaction);

pub struct GetTransactionById(String);

pub struct GetAllTransactions;

/*#[get("/transactions/<id>")]
fn get_transactions(id: &str) -> Json<Transaction> {
    unimplemented!()
}*/

/*#[post("/transactions", data="<transaction>")]
fn post_transaction(transaction: &Json<Transaction>) -> Json<Transaction> {
    unimplemented!()
}*/

/*#[launch]
fn rocket() {
    rocket::build().mount("/", routes![hello])
}*/

struct MessageSource;

struct MessageSink;

struct TxDatabase;

impl Actor for MessageSource {
    type Context = Context<Self>;
}

impl Actor for MessageSink {
    type Context = Context<Self>;
}

impl Actor for TxDatabase {
    type Context = Context<Self>;
}

impl Message for GetAllTransactions {
    type Result = ();
}

/*struct Envelope<M: Message + Sync + Send, T: Message + Sync + Send> where T::Result: Send + Sync {
    return_to: Recipient<T>,
    message: M
}*/

struct Envelope<M: Message + Sync + Send> {
    request_id: String,
    message: M
}

impl<M: Message + Sync + Send> Message for Envelope<M> {
    type Result = M::Result;
}

impl Handler<Envelope<GetAllTransactions>> for TxDatabase {
    type Result = ();

    fn handle(&mut self, _: Envelope<GetAllTransactions>, _: &mut <Self as actix::Actor>::Context) -> () { 
        todo!() 
    }
}

//fn main() {
//    println!("Hello, world!");
//}
