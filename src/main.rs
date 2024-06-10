#[macro_use]extern crate rocket;
pub mod db;
pub mod model;
pub mod controller;
use db::mongo_db::MongoDb;
use rocket::fs::FileServer;
use crate::controller::user_controller::{create,fetch,login};


// #[launch]
// async fn rocket()->_{
//     let dbase=MongoDb::init_db().await;
//     rocket::build().manage(dbase)
//     .mount("/", routes![create,fetch,login])
// }

use rocket::{config::Config, routes};
#[rocket::main]
async fn main()->Result<(),rocket::Error>{
    let db=MongoDb::init_db().await;
    let config=Config::figment().merge(("port",8000))
    .merge(("address","0.0.0.0"));

    let _=rocket::custom(config).manage(db)
    .mount("/", FileServer::from("src/static"))
    .mount("/", routes![create,fetch,login])
    .launch()
    .await?;

    Ok(())
}