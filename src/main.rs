use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::{self, OptsBuilder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"));
     // Connect to the MySQL database
     let mut conn = mysql::Conn::new(
        OptsBuilder::new()
            .user(Some("root"))
            .pass(Some("19992000Cc."))
            .db_name(Some("mydatabase")),)

    .unwrap();
 
 // Execute a simple query
    let rows = conn.query_iter("SELECT * FROM mydatabase.users WHERE id = 1").unwrap();
    let result = rows
         .map(|row| row.unwrap())
         .collect::<Vec<_>>()
         .pop()
         .unwrap();

    HttpResponse::Ok().body(format!("Result: {:?}", result))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}