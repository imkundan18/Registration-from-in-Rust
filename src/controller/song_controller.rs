use crate::db::mongo_db::MongoDb;
use crate::model::user_model::Song;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson},
    results::{DeleteResult, InsertOneResult},
};
use rocket::{http::Status, serde::json};
use rocket::serde::json::Json;
use rocket::State;

//Resgister for Admin
#[post("/add_song", format = "json", data = "<register>")]
pub async fn add_song(db: &State<MongoDb>,register: Json<Song>,) -> Result<Json<InsertOneResult>, Status> {
    let new_song = register.into_inner();

    let filter = doc! {
        "name": &new_song.name,
        "link": &new_song.link,
    };
    let existing_song = db.coll3.find_one(filter, None).await;

    match existing_song {
        Ok(Some(_)) => {
            // If a duplicate is found, return a custom error message
            Err(Status::Conflict) // 409 Conflict
        }
        Ok(None) => {
            let inst = db.coll3.insert_one(&new_song, None).await;
            match inst {
                Ok(res) => Ok(Json(res)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_data")]
pub async fn get_data(db: &State<MongoDb>) -> Json<Vec<Song>> {
    let mut cursor = db.coll3.find(None, None).await.unwrap();
    let mut list = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        list.push(result);
    }
    Json(list)
}

 #[delete("/delete_song/<name>")]
 pub async fn delete_song(db: &State<MongoDb>, name:&str) -> Result<String, Status> {
    // let object_id = ObjectId::with_string(&id).map_err(|_| Status::BadRequest)?;
    let aname=name;

//     let filter = doc! { "_id": object_id };
    let filter=doc! {"name":aname};

     let result = db.coll3.delete_one(filter, None).await;

    match result {
        Ok(res) =>{
            if res.deleted_count>0{
            Ok(String::from("Song Deleted"))
        }else{
            Err(Status::NotFound)
        }
    },
        Err(_) => Err(Status::InternalServerError),
   }
 }
