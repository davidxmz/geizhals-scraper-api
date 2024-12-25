// Beschreibung: Standardantworten für erfolgreiche und fehlerhafte Anfragen

// Importieren der benötigten Bibliotheken
use serde::{Deserialize, Serialize};

// Standardantwort für erfolgreiche Anfragen
#[derive(Serialize, Deserialize)]
pub struct DefaultResponse {
    pub data: serde_json::Value,
    pub code: u16,
}

// Standardantwort für fehlerhafte Anfragen
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: serde_json::Value,
    pub code: u16,
}
