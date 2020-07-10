use crate::models;
use crate::services;
use actix_web::{web, HttpResponse, Responder, Scope};

async fn create() -> impl Responder {
    return HttpResponse::Created().json(models::dto::todo::TodoDTO {
        id: String::from("0"),
    });
}

async fn get_all() -> impl Responder {
    let data = services::todo::get_all();

    return HttpResponse::Ok().json(data);
}

async fn get_one(params: web::Path<models::dto::todo::TodoDTO>) -> impl Responder {
    return HttpResponse::Ok().json(services::todo::get_one(params.id.to_string()));
}

async fn delete(_params: web::Path<models::dto::todo::TodoDTO>) -> impl Responder {
    return HttpResponse::NoContent();
}

pub fn controller() -> Scope {
    web::scope("/todo")
        // .wrap(middleware::NormalizePath) what's going on here?
        .route("/", web::post().to(create))
        .route("/", web::get().to(get_all))
        .route("/{id}", web::get().to(get_one))
        .route("/{id}", web::delete().to(delete))
}
