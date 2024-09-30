use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Album {
    id: String,
    title: String,
    artist: String,
    price: f64,
}

// Initial list of albums
struct AppState {
    albums: Mutex<Vec<Album>>,
}

// Get all albums
async fn get_albums(data: web::Data<AppState>) -> impl Responder {
    let albums = data.albums.lock().unwrap();
    HttpResponse::Ok().json(&*albums)
}

// Post a new album
async fn post_album(album: web::Json<Album>, data: web::Data<AppState>) -> impl Responder {
    let mut albums = data.albums.lock().unwrap();
    albums.push(album.into_inner());
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let albums = vec![
        Album { id: "1".to_string(), title: "Blue Train".to_string(), artist: "John Coltrane".to_string(), price: 56.99 },
        Album { id: "2".to_string(), title: "Jeru".to_string(), artist: "Gerry Mulligan".to_string(), price: 17.99 },
        Album { id: "3".to_string(), title: "Sarah Vaughan and Clifford Brown".to_string(), artist: "Sarah Vaughan".to_string(), price: 39.99 },
    ];

    let app_data = web::Data::new(AppState {
        albums: Mutex::new(albums),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/albums", web::get().to(get_albums))
            .route("/albums", web::post().to(post_album))
    })
    .bind("localhost:8080")?
    .run()
    .await
}