// Beschreibung: Hauptdatei des Projekts, die die Suche nach Produkten startet und die Ergebnisse speichert

// Importieren der benötigten Module
mod api;
mod defaults;
mod models;
mod routes;
mod scraper;
mod utils;

// Importieren der benötigten Bibliotheken
use dialoguer::{Input, Select};
use std::process;

// Hauptfunktion
#[tokio::main]
async fn main() {
    // Begrüßungsnachricht
    println!("Willkommen zum Preisvergleichsprogramm!");
    println!("Powered by Geizhals!");
    println!("");

    // Menü anzeigen
    let select_options: Vec<&str> = vec!["API starten", "Einzelne Suchanfrage", "Beenden"];

    let selection: usize = Select::new()
        .with_prompt("Bitte wählen Sie eine Option")
        .items(&select_options)
        .default(0)
        .interact()
        .expect("Fehler beim Anzeigen des Menüs");

    // Auswahl verarbeiten
    match selection {
        0 => {
            //API starten
            println!("API wird hochgefahren...");
            if let Err(e) = api::startup_api().await {
                eprintln!("Fehler beim Starten der API: {}", e);
                process::exit(1);
            }
        }
        1 => {
            //Einzelanfrage starten
            println!("Starte eine Einzelanfrage an Geizhals...");
            println!("Das Ergebnis wird in einer CSV-Datei gespeichert.");
            einzelanfrage().await;
        }
        2 => {
            //Programm beenden
            println!("Programm wird beendet...");
            process::exit(0);
        }
        _ => unreachable!(),
    }
}

// Funktion für die Einzelanfrage
async fn einzelanfrage() {
    // Benutzereingaben
    println!("Bitte geben Sie den Pfad für den Ausgabeordner ein:");
    let mut path: String = Input::new()
        .with_prompt("Pfad zum Ausgabeordner")
        .interact()
        .expect("Fehler beim Einlesen des Pfads");

    println!("Bitte geben Sie die Suchanfrage ein:");
    let search_query: String = Input::new()
        .with_prompt("Suchanfrage")
        .interact()
        .expect("Fehler beim Einlesen der Suchanfrage");

    // Geizhals abfragen und nach angegebenem Produkt suchen
    let results: Vec<models::Product> = scraper::scrape_geizhals(&search_query).await.unwrap();

    // Pfad formatieren
    if path.ends_with('/') {
        path.pop();
    }

    path = format!("{}/products-{}.csv", &path, &search_query);

    // Ergebnisse speichern
    if let Err(e) = utils::save_to_csv(&path, &results) {
        println!("ERROR: Fehler beim Speichern der Daten: {}", e);
    } else {
        // Erfolgsmeldung
        eprintln!("Erfolgreich gespeichert in {}!", &path);

        // Kompletten Pfad ausgeben falls relativ
        if path.starts_with("./") {
            eprintln!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                path.trim_start_matches("./")
            );
        }
    }
}
