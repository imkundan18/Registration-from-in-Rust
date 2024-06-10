use mongodb::{bson::{doc, to_bson}, results::InsertOneResult};
use rocket::State;
use crate::db::mongo_db::MongoDb;
use rocket::serde::json::Json;
use crate::model::user_model::{Student,StudentResponse};
use rocket::http::Status;
use futures::stream::StreamExt;

use rocket::{get,put};



//Resgister by name,Email,password
#[post("/register", format = "json", data = "<register>")]
pub async fn create(db:&State<MongoDb>, register:Json<Student>)->Result<Json<InsertOneResult>,Status>{

    let new_create=register.into_inner();
    let inst=db.coll.insert_one(&new_create, None).await;
    let json = match inst {

        Ok(res)=>Ok(Json(res)),
        Err(_)=>Err(Status::InternalServerError),
    };
    json
}
//Access by using Enail and password
#[post("/featch", format = "json", data = "<detail>")]
pub async fn fetch(db:&State<MongoDb>,detail:Json<Student>)->Result<Json<StudentResponse>,Status>{

    let val=detail.into_inner();
    let password=val.password;
    let email=val.email;

    let filter=doc! {"email":email,"password":password};
    //let collection =db.coll::<Student>("students_detail");
 
    let mut curser=db.coll.find(filter,None).await.map_err(|_| Status::InternalServerError)?;
    if let Some(result) = curser.next().await {
        match result {
            Ok(student) => {
                if let Some(name) = student.name {
                    //if let (Some(name), Some(roll), Some(subject)) = (student.name, student.roll, student.subject) {
                    //let response = StudentResponse { name, roll, subject };
                    let response = StudentResponse {name};
                    Ok(Json(response))
                } else {
                    Err(Status::InternalServerError)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::NotFound)
    }
}


//#[get("/login/<email>/<password>")] // direct in url :http://127.0.0.1:8000/login/Kumar@gmail.com/Kumar123
#[get("/login?<email>&<password>")] //use for Query param: http://127.0.0.1:8000/login?email=Kumar@gmail.com&password=Kumar123
pub async fn login(email:String,password:String, db:&State<MongoDb>)->String{

    let filter=doc! {"email":email,"password":password};
    let curser=db.coll.find(filter,None).await;
        match curser 
        {
            //Ok(curser)=> curser,
            Ok(_) => "Found".to_string(),
            Err(_)=>return "Error finding user".to_string(),
        };
         if let Some(result) = curser.expect("REASON").next().await {
            match result {
                Ok(_) => "Found".to_string(),
                Err(_) => "Error processing result".to_string(),
            }
        } else {
            "Not Found".to_string()
        }

}

#[get("/get_data")]
pub async fn get_data(db: &State<MongoDb>) -> Result<Json<Vec<Student>>, Status> {
    let mut students = Vec::new();
    let curs = db.coll.find(None, None).await.map_err(|_| Status::InternalServerError)?;
    
    while let Some(result) = curs.try_next().await.map_err(|_| Status::InternalServerError)? {
        let doc = result;
        let student = Student {
            id: doc.get_object_id("_id"),
            name: doc.get_str("name").unwrap_or("N/A").to_string(),
            email: doc.get_str("email").unwrap_or("N/A").to_string(),
            password: doc.get_str("password").unwrap_or("N/A").to_string(),
        };
                students.push(student);
            }
        

    Ok(Json(students))
}


#[put("/update/<name>",format = "json", data="<udata>")]
pub async fn update_data(db:&State<MongoDb>, udata:Json<Student>, name:String)->Result<Json<Student>,Status>{

    let uname=name;
    if uname.is_empty(){
        return Err(Status::BadRequest)
    };
    let data=Student{
        id:None,
        name:udata.name.to_owned(),
        email:udata.email.to_owned(),
        password:udata.password.to_owned()
    };
    let filter=doc! {"name":uname};
    let bson_data=to_bson(&data).unwrap();
    let filter2=doc! {"$set":bson_data};
    let res=db.coll.update_one(filter, filter2, None).await;
    match res{
        Ok(_)=>Ok(Json(data)),
        Err(_)=>Err(Status::BadRequest),

    }
}

    
