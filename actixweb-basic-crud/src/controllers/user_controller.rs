use crate::db::PgPool;
use crate::model::schema::users::dsl::*;
use crate::model::user::{NewUser, UpdateUser, User};
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    };
    let result = web::block(move || users.load::<User>(&mut conn)).await;
    match result {
        Ok(Ok(list)) => HttpResponse::Ok().json(json!({"message":"Users retrieved!","users": list})),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn get_by_id(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    };
    let result = web::block(move || users.find(user_id).first::<User>(&mut conn)).await;
    match result {
        Ok(Ok(u)) => HttpResponse::Ok().json(json!({"message":"User retrieved!","user": u})),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn create(payload: web::Json<NewUser>, pool: web::Data<PgPool>) -> impl Responder {
    let new_user = payload.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    };
    let result = web::block(move || {
        diesel::insert_into(users).values(&new_user).get_result::<User>(&mut conn)
    })
    .await;
    match result {
        Ok(Ok(u)) => HttpResponse::Ok().json(json!({"message":"User created!","user": u})),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn update(path: web::Path<Uuid>, payload: web::Json<UpdateUser>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = path.into_inner();
    let updates = payload.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    };
    let result = web::block(move || {
        diesel::update(users.find(user_id)).set(&updates).get_result::<User>(&mut conn)
    })
    .await;
    match result {
        Ok(Ok(u)) => HttpResponse::Ok().json(json!({"message":"User updated!","user": u})),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

pub async fn delete(path: web::Path<Uuid>, pool: web::Data<PgPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    };
    let result = web::block(move || diesel::delete(users.find(user_id)).execute(&mut conn)).await;
    match result {
        Ok(Ok(affected)) if affected > 0 => HttpResponse::Ok().json(json!({"message":"User deleted!"})),
        Ok(Ok(_)) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}