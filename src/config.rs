use serde::{Deserialize, Serialize};

pub type Config = Vec<Activity>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Boolean,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NotificationType {
    #[serde(rename = "notify-send")]
    NotifySend,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub fields: Vec<Field>,
    #[serde(default)]
    pub notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    #[serde(alias = "type")]
    pub field_type: FieldType,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub at: String,
    pub message: String,
    #[serde(alias = "type")]
    pub notfication_type: NotificationType,
}
