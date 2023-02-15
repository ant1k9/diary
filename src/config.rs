use serde::{Deserialize, Serialize};

pub type Config = Vec<Activity>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NotificationType {
    #[serde(rename = "notify-send")]
    NotifySend,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub fields: Vec<Field>,
    #[serde(default)]
    pub notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub at: String,
    pub message: String,
    #[serde(alias = "type")]
    pub notfication_type: NotificationType,
}
