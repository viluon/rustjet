/// Ticket business logic service
use chrono::Utc;

use crate::domain::DomainTicket;

/// Ticket service - business logic for ticket operations
pub struct TicketService;

impl TicketService {
    pub fn new() -> Self {
        Self
    }

    /// Filter tickets departing within specified hours
    pub fn get_upcoming_tickets(tickets: &[DomainTicket], hours: i64) -> Vec<&DomainTicket> {
        let now = Utc::now();
        let cutoff = now + chrono::Duration::hours(hours);

        tickets
            .iter()
            .filter(|ticket| {
                let departure = ticket.route.departure_time;
                departure >= now && departure <= cutoff
            })
            .collect()
    }
}

impl Default for TicketService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Money, Route, TicketStatus};
    use chrono::Utc;

    fn mock_ticket(hours_from_now: i64) -> DomainTicket {
        let departure_time = Utc::now() + chrono::Duration::hours(hours_from_now);
        DomainTicket {
            id: 1,
            ticket_code: "TEST123".to_string(),
            route: Route {
                from: "Prague".to_string(),
                to: "Brno".to_string(),
                departure_time,
                arrival_time: departure_time + chrono::Duration::hours(2),
            },
            price: Money {
                amount: 10.50,
                currency: "CZK".to_string(),
            },
            state: TicketStatus::Valid,
        }
    }

    #[test]
    fn test_get_upcoming_tickets() {
        let tickets = vec![mock_ticket(2), mock_ticket(25)];

        let upcoming = TicketService::get_upcoming_tickets(&tickets, 24);
        assert_eq!(upcoming.len(), 1);
        assert_eq!(upcoming[0].id, tickets[0].id);
    }

    #[test]
    fn test_get_upcoming_tickets_empty() {
        let tickets = vec![mock_ticket(30), mock_ticket(40)];

        let upcoming = TicketService::get_upcoming_tickets(&tickets, 24);
        assert_eq!(upcoming.len(), 0);
    }

    #[test]
    fn test_get_upcoming_tickets_all() {
        let tickets = vec![mock_ticket(1), mock_ticket(5), mock_ticket(10)];

        let upcoming = TicketService::get_upcoming_tickets(&tickets, 24);
        assert_eq!(upcoming.len(), 3);
    }
}
