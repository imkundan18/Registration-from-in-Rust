use mongodb::{Client, Collection};
use crate::model::user_model::{Student};

#[derive()]
pub struct MongoDb{
    pub coll:Collection<Student>,
}

impl MongoDb {
    pub async fn init_db() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database=client.database("Student");
        let coll=database.collection::<Student>("student_detail");
        MongoDb{coll}

    }
}