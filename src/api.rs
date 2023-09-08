use actix_web::{HttpResponse, Responder};

pub fn process_data() -> String {
    // Votre logique de traitement des données ici
    "Resultat du traitement des données".to_string()
}

pub async fn index() -> impl Responder {
    let result = process_data();
    HttpResponse::Ok().body(result)
}