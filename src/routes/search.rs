// Beschreibung: Enthält die Route für die Suchfunktion

// Importieren der benötigten Bibliotheken
use crate::{defaults::DefaultResponse, defaults::ErrorResponse, models::Product, scraper};
use actix_web::{post, HttpResponse, Responder};

// POST /search
#[post("/search")]
pub async fn post_search(req_body: String) -> impl Responder {
    // Legt die Standardantworten fest
    let mut def_res: DefaultResponse = DefaultResponse {
        data: serde_json::json!("Keine Ergebnisse vorhanden!"),
        code: 404,
    };

    let mut err_res: ErrorResponse = ErrorResponse {
        error: serde_json::json!("Es ist ein unbekannter Fehler aufgetreten!"),
        code: 500,
    };

    // Parst die Anfrage
    let query: serde_json::Value = match serde_json::from_str(&req_body) {
        Ok(val) => val,
        Err(_) => {
            err_res.error = serde_json::json!("Probleme bei der JSON-Verarbeitung!");
            err_res.code = 500;
            return HttpResponse::InternalServerError().json(&err_res);
        }
    };

    // Überprüft, ob die Anfrage gültig ist
    let search_query: String = match query.get("query") {
        Some(val) => val.to_string(),
        None => {
            err_res.error = serde_json::json!("Es wurde keine gültige Suchanfrage angegeben!");
            err_res.code = 400;
            return HttpResponse::BadRequest().json(&err_res);
        }
    };

    // Gibt die Suchanfrage aus
    println!("Neue Suchanfrage: {}", search_query);

    // Startet den Scraper
    let results: Vec<Product> = match scraper::scrape_geizhals(&search_query).await {
        Ok(res) => res,
        Err(_) => {
            err_res.error = serde_json::json!("Fehler beim Scrapen der Daten!");
            err_res.code = 500;
            return HttpResponse::InternalServerError().json(&err_res);
        }
    };

    // Überprüft, ob Ergebnisse existieren
    if !results.is_empty() {
        def_res.data = serde_json::json!(results);
        def_res.code = 200;
    }

    // Gibt die Ergebnisse zurück
    HttpResponse::Ok().json(&def_res)
}
