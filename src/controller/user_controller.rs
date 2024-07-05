use mongodb::{bson::{doc, to_bson}, results::InsertOneResult};
use rocket::{response::status, State};
use crate::db::mongo_db::MongoDb;
use rocket::serde::json::Json;
use crate::model::user_model::{Admin,User};
use rocket::http::Status;
use futures::stream::StreamExt;

//Resgister for Admin
#[post("/admin", format = "json", data = "<register>")]
pub async fn create_admin(db:&State<MongoDb>, register:Json<Admin>)->Result<Json<InsertOneResult>,Status>{

    let new_admin=register.into_inner();
    let inst=db.coll.insert_one(&new_admin, None).await;
    let json = match inst {

        Ok(res)=>Ok(Json(res)),
        Err(_)=>Err(Status::InternalServerError),
    };
    json
}
//Resgister for User
#[post("/user", format = "json", data = "<register>")]
pub async fn create_user(db:&State<MongoDb>, register:Json<User>)->Result<Json<InsertOneResult>,Status>{

    let new_user=register.into_inner();//It extracts and returns the inner data that was wrapped by the outer type.
    let inst=db.coll2.insert_one(&new_user, None).await;
    let json = match inst {

        Ok(res)=>Ok(Json(res)),
        Err(_)=>Err(Status::InternalServerError),
    };
    json
}
//Login User by using Email and password
 #[post("/user_login", format = "json", data = "<detail>")]
 pub async fn user_login(db:&State<MongoDb>,detail:Json<User>)->Result<Json<String>,Status>{

     let val=detail.into_inner();
     let password=val.password;
     let email=val.email;

     let filter=doc! {"email":email,"password":password};
     let mut curser=db.coll2.find(filter,None).await.map_err(|_| Status::InternalServerError)?;
     while let Some(result) = curser.next().await {
         match result {
             Ok(_) => return Ok(Json(format!("Correct"))),              
             Err(_) => return Err(Status::InternalServerError),
         }
        }
        Err(Status::NotFound)
    }

//Login Admin by using Email, password and bool
#[post("/admin_login", format = "json", data = "<detail>")]
pub async fn admin_login(db:&State<MongoDb>,detail:Json<Admin>)->Result<Json<String>,Status>{

    let val=detail.into_inner();
    let email=val.email;
    let password=val.password;
    let admin=val.admin;
if !admin{
    return Err(Status::Forbidden);
}else{
    let filter=doc! {"email":email,"password":password,};
    let mut curser=db.coll.find(filter,None).await.map_err(|_| Status::InternalServerError)?;
    while let Some(result) = curser.next().await {
        match result {
            Ok(_) => return Ok(Json(format!("Correct"))), 
            //  Redirect::to("/welcome")           
            Err(_) => return Err(Status::InternalServerError),
        }
       }

       Err(Status::NotFound)
   }
}

#[put("/update_password/<email>",data="<new_data>")]
pub async fn update_p(db:&State<MongoDb>,new_data:Json<User>,email:String)->Result<String,Status>{
    
    if email.is_empty(){
        return Ok(String::from("Email not provided"));
    }
    let filter=doc! {"email":&email};
    let existing=db.coll2.find_one(filter.clone(), None).await;
    match existing{
        Ok(Some(_)) => {
            let updata_data=doc! {"$set":{"password":&new_data.password}};
        
        let upd = db.coll2.update_one(filter, updata_data, None).await;
        match upd {
        Ok(result) => {
            if result.matched_count > 0 {
                Ok(String::from("Password updated successfully"))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
 
// //#[get("/login/<email>/<password>")] // direct in url :http://127.0.0.1:8000/login/Kumar@gmail.com/Kumar123
// #[get("/login?<email>&<password>")] //use for Query param: http://127.0.0.1:8000/login?email=Kumar@gmail.com&password=Kumar123
// pub async fn login(email:String,password:String, db:&State<MongoDb>)->String{

//     let filter=doc! {"email":email,"password":password};
//     let curser=db.coll.find(filter,None).await;
//         match curser 
//         {
//             //Ok(curser)=> curser,
//             Ok(_) => "Found".to_string(),
//             Err(_)=>return "Error finding user".to_string(),
//         };
//          if let Some(result) = curser.expect("REASON").next().await {
//             match result {
//                 Ok(_) => "Found".to_string(),
//                 Err(_) => "Error processing result".to_string(),
//             }
//         } else {
//             "Not Found".to_string()
//         }

// }

// #[get("/get_data")]
// pub async fn get_data(db: &State<MongoDb>) -> Result<Json<Vec<Student>>, Status> {
//     let mut students = Vec::new();
//     let curs = db.coll.find(None, None).await.map_err(|_| Status::InternalServerError)?;
    
//     while let Some(result) = curs.try_next().await.map_err(|_| Status::InternalServerError)? {
//         let doc = result;
//         let student = Student {
//             id: doc.get_object_id("_id"),
//             name: doc.get_str("name").unwrap_or("N/A").to_string(),
//             email: doc.get_str("email").unwrap_or("N/A").to_string(),
//             password: doc.get_str("password").unwrap_or("N/A").to_string(),
//         };
//                 students.push(student);
//             }
        

//     Ok(Json(students))
// }


// #[put("/update/<name>",format = "json", data="<udata>")]
// pub async fn update_data(db:&State<MongoDb>, udata:Json<Student>, name:String)->Result<Json<Student>,Status>{

//     let uname=name;
//     if uname.is_empty(){
//         return Err(Status::BadRequest)
//     };
//     let data=Student{
//         id:None,
//         name:udata.name.to_owned(),
//         email:udata.email.to_owned(),
//         password:udata.password.to_owned()
//     };
//     let filter=doc! {"name":uname};
//     let bson_data=to_bson(&data).unwrap();
//     let filter2=doc! {"$set":bson_data};
//     let res=db.coll.update_one(filter, filter2, None).await;
//     match res{
//         Ok(_)=>Ok(Json(data)),
//         Err(_)=>Err(Status::BadRequest),

//     }
// }

    
