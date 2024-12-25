// Beschreibung: Startet die API und bindet die Routen ein

// Importieren der benötigten Bibliotheken
use crate::routes;
use actix_web::{App, HttpServer};

// Startet die API
pub async fn startup_api() -> Result<(), std::io::Error> {
    let address: &str = "0.0.0.0";
    let port: u16 = 8081;
    println!("Starte...");
    let server = HttpServer::new(move || App::new().service(routes::search::post_search))
        .bind((address, port))?
        .run();
    println!("API erfolgreich gestartet auf http://{}:{}", address, port);
    server.await
}
