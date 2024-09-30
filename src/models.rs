use chrono::NaiveDate;
use diesel::prelude::{Associations, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transaction_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(belongs_to(TransactionCategory, foreign_key = category_id))]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,
    pub transaction_number: String,
    pub amount: f32,
    pub category_id: i32,
}
