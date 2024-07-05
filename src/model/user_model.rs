use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize,Debug)]
pub struct Admin{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    
    pub id:Option<ObjectId>,
    pub name:Option<String>,
    pub email:String,
    pub password:String,
    pub admin:bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    
    pub id:Option<ObjectId>,
    pub name:Option<String>,
    pub email:String,
    pub password:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Song{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    
    pub id:Option<ObjectId>,
    pub name:Option<String>,
    pub link:String,
}
