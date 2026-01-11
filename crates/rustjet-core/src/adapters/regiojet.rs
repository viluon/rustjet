/// RegioJet API adapter - implements TicketRepository port
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use regiojet_api::{
    apis::{
        configuration::{ApiKey, Configuration},
        tickets_api::get_all_tickets,
        users_api::login_registered_account,
    },
    models::{RegisteredLogin, Ticket, TicketState, Token},
};

use crate::{
    domain::{DomainTicket, Money, Route, TicketStatus, UserCredentials},
    ports::TicketRepository,
};

/// RegioJet API adapter
#[derive(Default)]
pub struct RegioJetAdapter;

impl RegioJetAdapter {
    pub fn new() -> Self {
        Self
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
}

#[async_trait]
impl TicketRepository for RegioJetAdapter {
    async fn fetch_tickets(&self, creds: &UserCredentials) -> Result<Vec<DomainTicket>> {
        let config = Self::login(&creds.account_code, &creds.password).await?;
        let tickets = get_all_tickets(
            &config, None, None, None, None, None, None, None, None, None, None, None, None, None,
        )
        .await
        .context("Failed to fetch tickets")?;

        Ok(tickets.into_iter().map(Into::into).collect())
    }
}

/// Convert RegioJet API ticket to domain ticket
impl From<Ticket> for DomainTicket {
    fn from(ticket: Ticket) -> Self {
        let route = extract_route(&ticket);
        let price = Money {
            amount: ticket.price,
            currency: ticket.currency.to_string(),
        };
        let state = convert_ticket_state(&ticket.state);

        DomainTicket {
            id: ticket.id,
            ticket_code: ticket.ticket_code,
            route,
            price,
            state,
        }
    }
}

/// Extract route from ticket sections
fn extract_route(ticket: &Ticket) -> Route {
    let sections = &ticket.route_sections;
    if sections.is_empty() {
        return Route {
            from: "Unknown".to_string(),
            to: "Unknown".to_string(),
            departure_time: Utc::now(),
            arrival_time: Utc::now(),
        };
    }

    let first = &sections[0].section;
    let last = &sections[sections.len() - 1].section;

    let from = first
        .departure_city_name
        .as_ref()
        .or(first.departure_station_name.as_ref())
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());

    let to = last
        .arrival_city_name
        .as_ref()
        .or(last.arrival_station_name.as_ref())
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());

    let departure_time = DateTime::parse_from_rfc3339(&first.departure_time)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);

    let arrival_time = DateTime::parse_from_rfc3339(&last.arrival_time)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);

    Route {
        from,
        to,
        departure_time,
        arrival_time,
    }
}

/// Convert API ticket state to domain ticket status
fn convert_ticket_state(state: &TicketState) -> TicketStatus {
    match state {
        TicketState::Valid => TicketStatus::Valid,
        TicketState::Canceled => TicketStatus::Cancelled,
        TicketState::Expired => TicketStatus::Expired,
        _ => TicketStatus::Unknown(format!("{:?}", state)),
    }
}
