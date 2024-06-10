use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize,Debug)]
pub struct Student{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    
    pub id:Option<ObjectId>,
    pub name:Option<String>,
    pub email:String,
    pub password:String,
}

#[derive(Serialize)]
pub struct StudentResponse{
    pub name:String,
}
