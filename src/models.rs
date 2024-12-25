// Beschreibung: Enthält die Struktur der Objekte, die in der Anwendung verwendet werden.

// Importieren der benötigten Bibliotheken
use chrono::DateTime;
use chrono_tz::Tz;
use serde::Serialize;
use serde::Serializer;

//Produkt-Struktur
#[derive(Debug, Serialize)]
pub struct Product {
    pub name: String,
    pub link: String,
    pub price: f64,
    pub image: String,
    pub offers: u32,
    #[serde(serialize_with = "serialize_dt")]
    pub date: DateTime<Tz>,
}

//Serialisiert das Datum
pub fn serialize_dt<S>(dt: &DateTime<Tz>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%d.%m.%Y %H:%M:%S %Z")
        .to_string()
        .serialize(serializer)
        .map_err(serde::ser::Error::custom)
}
