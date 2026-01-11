/// Telegram notification adapter
use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use teloxide::{prelude::*, types::ParseMode};

use crate::{domain::DomainTicket, ports::NotificationService};

/// Telegram notification service adapter
pub struct TelegramAdapter {
    bot: Bot,
}

impl TelegramAdapter {
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }

    /// Format tickets for Telegram message
    pub fn format_tickets_message(tickets: &[&DomainTicket]) -> String {
        if tickets.is_empty() {
            return "No upcoming tickets found.".to_string();
        }

        let mut message = format!("🎫 *Upcoming Tickets* ({})\n\n", tickets.len());

        for (idx, ticket) in tickets.iter().enumerate() {
            message.push_str(&format!("{}\\. ", idx + 1));
            message.push_str(&Self::format_ticket(ticket));
            message.push_str("\n\n");
        }

        message
    }

    /// Format single ticket
    fn format_ticket(ticket: &DomainTicket) -> String {
        let from = &ticket.route.from;
        let to = &ticket.route.to;
        let departure_time = Self::format_datetime(&ticket.route.departure_time.to_rfc3339());
        let arrival_time = Self::format_datetime(&ticket.route.arrival_time.to_rfc3339());
        let price = format!("{:.2} {}", ticket.price.amount, ticket.price.currency);
        let state = format!("{:?}", ticket.state);

        format!(
            "*{}* → *{}*\n⏰ {} → {}\n💰 {}\n📋 {}",
            Self::escape_markdown(from),
            Self::escape_markdown(to),
            Self::escape_markdown(&departure_time),
            Self::escape_markdown(&arrival_time),
            Self::escape_markdown(&price),
            Self::escape_markdown(&state)
        )
    }

    /// Format datetime string to readable format
    fn format_datetime(datetime_str: &str) -> String {
        DateTime::parse_from_rfc3339(datetime_str)
            .ok()
            .map(|dt| dt.format("%d/%m %H:%M").to_string())
            .unwrap_or_else(|| datetime_str.to_string())
    }

    /// Escape special characters for Telegram MarkdownV2
    fn escape_markdown(text: &str) -> String {
        text.chars()
            .map(|c| match c {
                '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '='
                | '|' | '{' | '}' | '.' | '!' => format!("\\{}", c),
                _ => c.to_string(),
            })
            .collect()
    }
}

#[async_trait]
impl NotificationService for TelegramAdapter {
    async fn send_message(&self, chat_id: i64, message: String) -> Result<()> {
        self.bot
            .send_message(ChatId(chat_id), message)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Money, Route, TicketStatus};
    use chrono::Utc;

    fn mock_ticket(from: &str, to: &str) -> DomainTicket {
        DomainTicket {
            id: 1,
            ticket_code: "TEST123".to_string(),
            route: Route {
                from: from.to_string(),
                to: to.to_string(),
                departure_time: Utc::now(),
                arrival_time: Utc::now(),
            },
            price: Money {
                amount: 10.50,
                currency: "CZK".to_string(),
            },
            state: TicketStatus::Valid,
        }
    }

    #[test]
    fn test_format_ticket() {
        let ticket = mock_ticket("Prague", "Brno");
        let formatted = TelegramAdapter::format_ticket(&ticket);

        assert!(formatted.contains("Prague"));
        assert!(formatted.contains("Brno"));
        assert!(formatted.contains("10\\.50"));
    }

    #[test]
    fn test_format_tickets_message_empty() {
        let tickets: Vec<&DomainTicket> = vec![];
        let message = TelegramAdapter::format_tickets_message(&tickets);
        assert_eq!(message, "No upcoming tickets found.");
    }

    #[test]
    fn test_escape_markdown() {
        assert_eq!(
            TelegramAdapter::escape_markdown("test_text*bold"),
            "test\\_text\\*bold"
        );
        assert_eq!(TelegramAdapter::escape_markdown("10.50"), "10\\.50");
    }
}
