use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::domains::alert::Alert;
use crate::repository::schema::alerts::dsl::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool
}

impl Database {

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_alert(&self, alert: Alert) -> Result<Alert, Error> {
        let new_alert = Alert {
            id: Option::None,
            source: alert.source,
            source_component: alert.source_component,
            description: Option::None,
            alert_type: alert.alert_type,
            subject_type: alert.subject_type,
            subject_reference_number: alert.subject_reference_number,
            subject_description: Option::Some(String::from("Subject description")),
            content: alert.content,
            created_at: Option::Some(Utc::now().naive_utc()),
            updated_at: Option::Some(Utc::now().naive_utc()),
        };
        diesel::insert_into(alerts)
            .values(&new_alert)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new alert");
        Ok(new_alert)
    }

    // pub fn get_todo_by_id(&self, todo_id: &str) -> Option<Todo> {
    //     let todo = todos
    //         .find(todo_id)
    //         .get_result::<Todo>(&mut self.pool.get().unwrap())
    //         .expect("Error loading todo by id");
    //     Some(todo)
    // }
    //
    // pub fn delete_todo_by_id(&self, todo_id: &str) -> Option<usize> {
    //     let count = diesel::delete(todos.find(todo_id))
    //         .execute(&mut self.pool.get().unwrap())
    //         .expect("Error deleting todo by id");
    //     Some(count)
    // }
    //
    //
    // pub fn update_todo_by_id(&self, todo_id: &str, mut todo: Todo) -> Option<Todo> {
    //     todo.updated_at = Some(Utc::now().naive_utc());
    //     let todo = diesel::update(todos.find(todo_id))
    //         .set(&todo)
    //         .get_result::<Todo>(&mut self.pool.get().unwrap())
    //         .expect("Error updating todo by id");
    //     Some(todo)
    // }
}