use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::{
    apis::{
        configuration::{ApiKey, Configuration},
        tickets_api::get_all_tickets,
        users_api::login_registered_account,
    },
    models::{RegisteredLogin, Ticket, Token},
};

/// Fetch user tickets from RegioJet API
pub async fn fetch_user_tickets(account_code: &str, password: &str) -> Result<Vec<Ticket>> {
    let config = login(account_code, password).await?;
    let tickets = get_all_tickets(
        &config, None, None, None, None, None, None, None, None, None, None, None, None, None,
    )
    .await
    .context("Failed to fetch tickets")?;
    Ok(tickets)
}

/// Login and get API configuration with auth token
async fn login(code: &str, password: &str) -> Result<Configuration> {
    let credentials = RegisteredLogin {
        account_code: code.to_string(),
        password: password.to_string(),
    };

    let Token { token } =
        login_registered_account(&Default::default(), credentials, None, None, None)
            .await
            .context("Login failed")?;

    Ok(Configuration {
        api_key: Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: token.context("No token received")?,
        }),
        ..Default::default()
    })
}

/// Filter tickets departing within specified hours
pub fn get_upcoming_tickets(tickets: &[Ticket], hours: i64) -> Vec<&Ticket> {
    let now = Utc::now();
    let cutoff = now + chrono::Duration::hours(hours);

    tickets
        .iter()
        .filter(|ticket| {
            if let Some(section) = ticket.route_sections.first() {
                if let Ok(departure) = DateTime::parse_from_rfc3339(&section.section.departure_time)
                {
                    let departure_utc = departure.with_timezone(&Utc);
                    return departure_utc >= now && departure_utc <= cutoff;
                }
            }
            false
        })
        .collect()
}

/// Format tickets for Telegram message
pub fn format_tickets_message(tickets: &[&Ticket]) -> String {
    if tickets.is_empty() {
        return "No upcoming tickets found.".to_string();
    }

    let mut message = format!("🎫 *Upcoming Tickets* ({})\n\n", tickets.len());

    for (idx, ticket) in tickets.iter().enumerate() {
        message.push_str(&format!("{}\\. ", idx + 1));
        message.push_str(&format_ticket(ticket));
        message.push_str("\n\n");
    }

    message
}

/// Format single ticket
fn format_ticket(ticket: &Ticket) -> String {
    let sections = &ticket.route_sections;
    if sections.is_empty() {
        return "Invalid ticket (no sections)".to_string();
    }

    let first = &sections[0].section;
    let last = &sections[sections.len() - 1].section;

    let from = first
        .departure_city_name
        .as_ref()
        .or(first.departure_station_name.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let to = last
        .arrival_city_name
        .as_ref()
        .or(last.arrival_station_name.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let departure_time = format_datetime(&first.departure_time);
    let arrival_time = format_datetime(&last.arrival_time);

    let price = format!("{:.2} {}", ticket.price, ticket.currency);
    let state = format!("{:?}", ticket.state);

    format!(
        "*{}* → *{}*\n⏰ {} → {}\n💰 {}\n📋 {}",
        escape_markdown(from),
        escape_markdown(to),
        escape_markdown(&departure_time),
        escape_markdown(&arrival_time),
        escape_markdown(&price),
        escape_markdown(&state)
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
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' => format!("\\{}", c),
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Currency, CustomerActions, Line, PassengersInfo, PriceConditions, Section, TicketSection,
        TicketState, VehicleType,
    };

    fn mock_section(
        departure_time: &str,
        arrival_time: &str,
        from: &str,
        to: &str,
    ) -> TicketSection {
        TicketSection {
            section: Box::new(Section {
                id: 1,
                vehicle_standard_key: "STANDARD".to_string(),
                support: false,
                support_code: None,
                vehicle_type: VehicleType::Bus,
                fixed_seat_reservation: true,
                line: Box::new(Line {
                    connection_id: 1,
                    line_code: "RJ".to_string(),
                    line_number: 123,
                    code: Some("123".to_string()),
                    from: "Prague".to_string(),
                    to: "Brno".to_string(),
                }),
                departure_station_id: 1,
                departure_station_name: None,
                departure_city_id: 1,
                departure_city_name: Some(from.to_string()),
                departure_time: departure_time.to_string(),
                departure_platform: None,
                arrival_station_id: 2,
                arrival_station_name: None,
                arrival_city_id: 2,
                arrival_city_name: Some(to.to_string()),
                arrival_time: arrival_time.to_string(),
                arrival_platform: None,
                carrier_id: None,
                free_seats_count: None,
                notices: None,
                services: None,
                delay: None,
                travel_time: "2h".to_string(),
                estimated_arrival_time: None,
                ..Default::default()
            }),
            fixed_seat_reservation: true,
            selected_seats: vec![],
            vehicles: None,
        }
    }

    fn mock_ticket(departure_time: &str) -> Ticket {
        Ticket {
            id: 1,
            ticket_code: "TEST123".to_string(),
            route_id: "1".to_string(),
            action_name: None,
            price: 10.50,
            unpaid: 0.0,
            currency: Currency::Czk,
            state: TicketState::Valid,
            seat_class_key: "STANDARD".to_string(),
            conditions: Box::new(PriceConditions::default()),
            expiration_date: None,
            expirate_at: None,
            customer_notifications: None,
            customer_actions: Box::new(CustomerActions::default()),
            route_sections: vec![mock_section(
                departure_time,
                "2026-01-11T12:00:00Z",
                "Prague",
                "Brno",
            )],
            addons: None,
            payment_id: None,
            bills: None,
            used_code_discount: None,
            used_percentual_discounts: None,
            transfers_info: None,
            surcharge: None,
            cancel_charge_sum: None,
            cancel_money_back_sum: None,
            passengers_info: Box::new(PassengersInfo::default()),
            delay: None,
            travel_time: None,
            estimated_arrival_time: None,
            affiliate_ticket: None,
            wheel_chair_platform_order_possible: false,
            ..Default::default()
        }
    }

    #[test]
    fn test_format_ticket() {
        let ticket = mock_ticket("2026-01-11T10:00:00Z");
        let formatted = format_ticket(&ticket);

        assert!(formatted.contains("Prague"));
        assert!(formatted.contains("Brno"));
        assert!(formatted.contains("10\\.50"));
    }

    #[test]
    fn test_format_tickets_message_empty() {
        let tickets: Vec<&Ticket> = vec![];
        let message = format_tickets_message(&tickets);
        assert_eq!(message, "No upcoming tickets found.");
    }

    #[test]
    fn test_get_upcoming_tickets() {
        let now = Utc::now();
        let in_2_hours = (now + chrono::Duration::hours(2)).to_rfc3339();
        let in_25_hours = (now + chrono::Duration::hours(25)).to_rfc3339();

        let tickets = vec![mock_ticket(&in_2_hours), mock_ticket(&in_25_hours)];

        let upcoming = get_upcoming_tickets(&tickets, 24);
        assert_eq!(upcoming.len(), 1);
    }

    #[test]
    fn test_escape_markdown() {
        assert_eq!(escape_markdown("test_text*bold"), "test\\_text\\*bold");
        assert_eq!(escape_markdown("10.50"), "10\\.50");
    }
}
