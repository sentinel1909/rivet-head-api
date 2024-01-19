// src/lib/routes/diary.rs

// dependencies
use crate::domain::appstate::AppState;
use crate::domain::diary_record::DiaryRecord;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Result,
};
use chrono::prelude::*;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

// struct to represent the incoming form data for a diary entry
#[derive(Debug, Deserialize)]
pub struct FormData {
    pub band_content: String,
    pub album_content: String,
    pub thoughts_content: String,
}

// struct to represent the incoming form data for an updated thoughts entry
#[derive(Debug, Deserialize)]
pub struct UpdatedThoughtsFormData {
    pub thoughts_content: String,
}

// struct to represent the incoming form data for an updated album entry
#[derive(Debug, Deserialize)]
pub struct UpdatedAlbumFormData {
    pub album_content: String,
}

// api/diary DELETE endpoint handler, deletes an entry with a specified ID from the database
#[tracing::instrument(name = "Delete Diary Entry", skip())]
#[delete("/diary/delete/{id}")]
async fn diary_delete(
    item_id: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = Uuid::from_str(&item_id).expect("Invalid UUID, cannot find record to delete...");
    let result = sqlx::query("DELETE FROM diary WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Diary item removed...".to_string())),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

// api/diary GET endpoint handler, returns all the diary entries stored in the diary database table
#[tracing::instrument(name = "Get Diary Entries", skip())]
#[get("/diary")]
async fn diary_get(state: web::Data<AppState>) -> Result<HttpResponse> {
    let result = sqlx::query_as::<_, DiaryRecord>("SELECT * FROM diary")
        .fetch_all(&state.pool)
        .await;

    match result {
        Ok(diary_entries) => Ok(HttpResponse::Ok().json(diary_entries)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

// api/diary POST endpoint handler, takes form data received for a new diary entry and stores it in the diary database table
#[tracing::instrument(name = "Add Diary Entry", skip())]
#[post("/diary/new")]
async fn diary_post(state: web::Data<AppState>, form: web::Form<FormData>) -> Result<HttpResponse> {
    let FormData {
        band_content,
        album_content,
        thoughts_content,
    } = form.0;
    let new_diary_record = DiaryRecord {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: None,
        band: band_content,
        album: album_content,
        thoughts: thoughts_content,
    };
    let result = sqlx::query(
        "INSERT INTO diary (id, created_at, updated_at, band, album, thoughts) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(new_diary_record.id)
    .bind(new_diary_record.created_at)
    .bind(new_diary_record.updated_at)
    .bind(new_diary_record.band)
    .bind(new_diary_record.album)
    .bind(new_diary_record.thoughts)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("New diary item added...".to_string())),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

// api/diary PUT endpoint handler, takes form data received for an updated album field and updates the corresponding item id specified in the query paramters
#[tracing::instrument(name = "Update Diary Entry - Album", skip())]
#[put("/diary/update/album/{id}")]
async fn diary_album_put(
    item_id: web::Path<String>,
    updated_album_content: web::Form<UpdatedAlbumFormData>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = Uuid::from_str(&item_id).expect("Invalid UUID, cannot find record to update...");

    let result = sqlx::query("UPDATE diary SET updated_at = $1, album = $2 WHERE id = $3")
        .bind(Some(Utc::now()))
        .bind(&updated_album_content.album_content)
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Diary item album content updated...".to_string())),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

// api/diary PUT endpoint handler, takes form data received for an updated thoughts field and updates the corresponding item id specified in the query paramters
#[tracing::instrument(name = "Update Diary Entry - Thoughts", skip())]
#[put("/diary/update/thoughts/{id}")]
async fn diary_thoughts_put(
    item_id: web::Path<String>,
    updated_thoughts_content: web::Form<UpdatedThoughtsFormData>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = Uuid::from_str(&item_id).expect("Invalid UUID, cannot find record to update...");

    let result = sqlx::query("UPDATE diary SET updated_at = $1, thoughts = $2 WHERE id = $3")
        .bind(Some(Utc::now()))
        .bind(&updated_thoughts_content.thoughts_content)
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json("Diary item thoughts content updated...".to_string())),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}
