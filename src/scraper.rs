// Beschreibung: Dieses Modul enthält die Funktion zum Scrapen von Geizhals.

// Importieren der benötigten Bibliotheken
use crate::models::Product;
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Berlin;
use chrono_tz::Tz;
use reqwest::Response;
use scraper::{Html, Selector};
use std::error::Error;

// Funktion zum Scrapen von Geizhals
pub async fn scrape_geizhals(query: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    // Website abrufen
    let url: String = format!("https://www.geizhals.de/?fs={}", query);
    let response: Response = reqwest::get(&url).await?;
    let body: String = response.text().await?;

    // HTML-Code parsen
    let document: Html = Html::parse_document(&body);

    // Selektoren definieren
    let product_selector: Selector = Selector::parse(".listview__item")?;
    let name_selector: Selector = Selector::parse(".listview__name-link")?;
    let price_selector: Selector = Selector::parse(".price")?;
    let image_selector: Selector = Selector::parse(".listview__image")?;
    let offers_selector: Selector = Selector::parse(".listview__offercount-link")?;

    // Erstellt einen Vektor für die Produkte
    let mut products: Vec<Product> = Vec::new();

    for element in document.select(&product_selector) {
        // Extrahiert und verarbeitet den Namen
        let name: String = element
            .select(&name_selector)
            .next()
            .map(|n| n.text().collect::<String>().replace("\n", ""))
            .unwrap_or_else(|| "Unbekannt".to_string());

        // Extrahiert und verarbeitet den Link
        let link: String = element
            .select(&name_selector)
            .next()
            .and_then(|p| p.value().attr("href"))
            .map(|href| format!("https://geizhals.de{}", href))
            .unwrap_or_else(|| "Link nicht verfügbar".to_string());

        // Extrahiert und verarbeitet den Preis
        let price: String = element
            .select(&price_selector)
            .next()
            .map(|p| {
                p.text()
                    .collect::<String>()
                    .replace("€", "")
                    .replace(" ", "")
                    .replace(",", ".")
            })
            .unwrap_or_else(|| "0.00".to_string());

        // Extrahiert und verarbeitet das Bild
        let image: String = element
            .select(&image_selector)
            .next()
            .and_then(|p| p.value().attr("src"))
            .map(|src| src.to_string())
            .unwrap_or_else(|| "Bild nicht verfügbar".to_string());

        // Extrahiert und verarbeitet die Angebote
        let offers: String = element
            .select(&offers_selector)
            .next()
            .map(|p| {
                p.text()
                    .collect::<Vec<_>>()
                    .concat()
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
            })
            .unwrap_or_else(|| "0".to_string());

        // Formatiert den Preis und die Angebote um
        let price: f64 = price.parse().unwrap_or(0.0);
        let offers: u32 = offers.parse().unwrap_or(0);

        // Generiert das aktuelle Datum
        let date: DateTime<Tz> = Utc::now().with_timezone(&Berlin);

        // Fügt das Produkt zum Vektor hinzu
        products.push(Product {
            name,
            link,
            price,
            image,
            offers,
            date,
        });
    }

    // Gibt eine Erfolgsmeldung aus
    println!("Suchanfrage erfolgreich verarbeitet!");

    // Gibt die Produkte zurück
    Ok(products)
}
