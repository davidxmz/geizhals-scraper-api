// Beschreibung: Hilfsfunktionen, die in der Anwendung verwendet werden

// Importieren der benötigten Bibliotheken
use crate::models::Product;
use std::fs::File;
use std::io::Write;

// Funktion zum Speichern der Produkte in einer CSV-Datei
pub fn save_to_csv(file_path: &str, products: &[Product]) -> Result<(), std::io::Error> {
    // Datei erstellen und Header schreiben
    let mut file = File::create(file_path)?;
    writeln!(file, "Name;Link;Preis;Bild;Angebote;Datum")?;

    // Produkte in die Datei schreiben
    for product in products {
        writeln!(
            file,
            "{};{};{};{};{};{:?}",
            product.name, product.link, product.price, product.image, product.offers, product.date
        )?;
    }

    Ok(())
}
