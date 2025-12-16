use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub private: bool,
    pub html_url: String,
    pub clone_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Issue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub user: User,
    pub labels: Vec<Label>,
    pub state: String,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    pub id: i64,
    pub body: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
}
