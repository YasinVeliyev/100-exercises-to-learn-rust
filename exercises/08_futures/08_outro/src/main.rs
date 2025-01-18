use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};
use outro_08::ticket::{TicketDraft, TicketStore};
use serde::*;

async fn create_ticket(ticket: web::Json<TicketDraft>) -> impl Responder {
    let mut store = TicketStore::new();
    store.add_ticket(ticket.into_inner());
    HttpResponse::Ok()
        .status(StatusCode::from_u16(201).unwrap())
        .json("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let connection = sqlite::open("../../progress.db");
    // let query = "SELECT * FROM sqlite_master";
    // match connection {
    //     Ok(connection) => connection
    //         .iterate(query, |pairs| {
    //             for &(name, value) in pairs.iter() {
    //                 println!("{} = {:?}", name, value);
    //             }
    //             true
    //         })
    //         .unwrap(),
    //     Err(_) => todo!(),
    // };

    HttpServer::new(|| App::new().route("/ticket", web::post().to(create_ticket)))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
