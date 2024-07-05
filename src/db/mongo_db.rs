use mongodb::{Client, Collection};
use crate::model::user_model::{Admin, Song, User};

#[derive()]
pub struct MongoDb{
    pub coll:Collection<Admin>,
    pub coll2:Collection<User>,
    pub coll3:Collection<Song>
}

impl MongoDb {
    pub async fn init_db() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database=client.database("Project");
        let coll=database.collection::<Admin>("admin_detail");
        let coll2=database.collection::<User>("User_detail");
        let coll3=database.collection::<Song>("Song_detail");
        MongoDb{coll,coll2,coll3}
    }
}