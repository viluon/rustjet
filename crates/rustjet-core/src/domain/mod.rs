/// Domain types - pure business models independent of external APIs
use chrono::{DateTime, Utc};

/// Domain representation of a ticket
#[derive(Debug, Clone, PartialEq)]
pub struct DomainTicket {
    pub id: i64,
    pub ticket_code: String,
    pub route: Route,
    pub price: Money,
    pub state: TicketStatus,
}

/// Route information for a ticket
#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub from: String,
    pub to: String,
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
}

/// Money with amount and currency
#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    pub amount: f64,
    pub currency: String,
}

/// Ticket status
#[derive(Debug, Clone, PartialEq)]
pub enum TicketStatus {
    Valid,
    Cancelled,
    Expired,
    Unknown(String),
}

/// User credentials for RegioJet account
#[derive(Debug, Clone, PartialEq)]
pub struct UserCredentials {
    pub account_code: String,
    pub password: String,
}

/// Telegram user information from WebApp initData
#[derive(Debug, Clone, PartialEq)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

/// Notification settings for a user
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationSettings {
    pub enabled: bool,
}
