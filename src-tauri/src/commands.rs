use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    id: i64,
    title: String,
    start: String,
    end: String,
    location: String,
    person_in_charge: String,
    contact_num: String,
    is_approved: i64,
    created_at: String,
    updated_at: String,
    color: String,
}

#[derive(Deserialize)]
pub struct CreateEvent {
    title: String,
    start: String,
    end: String,
    location: String,
    person_in_charge: String,
    contact_num: String,
    is_approved: i64,
    color: String,
}

#[derive(Deserialize)]
pub struct EditEvent {
    id: i64,
    title: String,
    start: String,
    end: String,
    location: String,
    person_in_charge: String,
    contact_num: String,
    is_approved: i64,
    color: String,
}

#[tauri::command]
pub async fn get_events(pool: State<'_, SqlitePool>) -> Result<Vec<Event>, String> {
    sqlx::query_as::<_, Event>("SELECT * FROM events")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_event_by_id(pool: State<'_, SqlitePool>, id: i64) -> Result<Event, String> {
    sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_event(pool: State<'_, SqlitePool>, event: CreateEvent) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO events (
            title, 
            start, 
            end, 
            location, 
            person_in_charge, 
            contact_num, 
            is_approved,
            color)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(event.title)
        .bind(event.start)
        .bind(event.end)
        .bind(event.location)
        .bind(event.person_in_charge)
        .bind(event.contact_num)
        .bind(event.is_approved)
        .bind(event.color)
        .execute(&*pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_event(pool: State<'_, SqlitePool>, event: EditEvent) -> Result<(), String> {
    sqlx::query(
        "UPDATE events 
         SET 
            title = ?, 
            start = ?, 
            end = ?, 
            location = ?, 
            person_in_charge = ?, 
            contact_num = ?, 
            is_approved = ?,
            color = ?
         WHERE id = ?")
                 .bind(event.title)
        .bind(event.start)
        .bind(event.end)
        .bind(event.location)
        .bind(event.person_in_charge)
        .bind(event.contact_num)
        .bind(event.is_approved)
        .bind(event.color)
        .bind(event.id)
        .execute(&*pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_event(pool: State<'_, SqlitePool>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM events WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}