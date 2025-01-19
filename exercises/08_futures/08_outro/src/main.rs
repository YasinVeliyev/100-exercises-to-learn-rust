use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, Responder};

use outro_08::ticket::{TicketDraft, TicketStore};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};

async fn read_file(path: &str) -> (fs::File, TicketStore) {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .await
        .unwrap();
    let mut tickets = String::new();
    let _ = file.read_to_string(&mut tickets).await;
    let store: TicketStore = serde_json::from_str(&tickets).unwrap();
    file.set_len(0).await.unwrap();
    file.seek(SeekFrom::Start(0)).await.unwrap();
    (file, store)
}

async fn create_ticket(ticket: web::Json<TicketDraft>) -> impl Responder {
    let (mut file, mut store) = read_file("../tickets.json").await;
    let ticket_draft = ticket.into_inner();
    let id = store.add_ticket(ticket_draft);
    let tickets = serde_json::to_vec(&store).unwrap();
    file.write(&tickets).await.unwrap();

    HttpResponse::Ok()
        .status(StatusCode::from_u16(201).unwrap())
        .json(id)
}

async fn get_all_tickets() -> impl Responder {
    let (_, store) = read_file("../tickets.json").await;

    HttpResponse::Ok()
        .status(StatusCode::from_u16(200).unwrap())
        .json(store)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/ticket", web::post().to(create_ticket))
            .route("/ticket", web::get().to(get_all_tickets))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
