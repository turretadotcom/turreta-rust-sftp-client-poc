// use serde::{Deserialize, Serialize};
use serde_derive::{Serialize,Deserialize };
use diesel::{Queryable, Insertable, AsChangeset, Identifiable};
use diesel::sql_types::BigInt;

#[derive(Default,Debug,Queryable,Identifiable, Insertable,Serialize,Deserialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::repository::schema::alerts)]
pub struct Alert {

    ///
    pub id: Option<i32>,
    pub source: String,
    pub source_component: String,
    pub description: Option<String>,
    pub alert_type: String,
    pub subject_type: String,
    pub subject_reference_number: String,
    pub subject_description: Option<String>,
    pub content: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>
}