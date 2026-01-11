use chrono::Utc;
use rustjet_core::{
    adapters::{json_storage::JsonCredentialsStorage, telegram::TelegramAdapter},
    domain::{DomainTicket, Money, Route, TicketStatus, UserCredentials},
    ports::CredentialsStorage,
    services::tickets::TicketService,
};

/// Create mock domain ticket for testing
fn mock_ticket(id: i64, hours_from_now: i64, from: &str, to: &str) -> DomainTicket {
    let departure_time = Utc::now() + chrono::Duration::hours(hours_from_now);
    DomainTicket {
        id,
        ticket_code: format!("TEST{}", id),
        route: Route {
            from: from.to_string(),
            to: to.to_string(),
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

#[tokio::test]
async fn test_ticket_filtering() {
    let tickets = vec![
        mock_ticket(1, 2, "Prague", "Brno"),
        mock_ticket(2, 25, "Brno", "Ostrava"),
        mock_ticket(3, 50, "Ostrava", "Warsaw"),
    ];

    // Filter for next 24 hours - should get only first ticket
    let upcoming_24h = TicketService::get_upcoming_tickets(&tickets, 24);
    assert_eq!(upcoming_24h.len(), 1);
    assert_eq!(upcoming_24h[0].id, 1);

    // Filter for next 48 hours - should get first two tickets
    let upcoming_48h = TicketService::get_upcoming_tickets(&tickets, 48);
    assert_eq!(upcoming_48h.len(), 2);
    assert_eq!(upcoming_48h[0].id, 1);
    assert_eq!(upcoming_48h[1].id, 2);

    // Filter for next 72 hours - should get all three tickets
    let upcoming_72h = TicketService::get_upcoming_tickets(&tickets, 72);
    assert_eq!(upcoming_72h.len(), 3);
}

#[tokio::test]
async fn test_format_tickets_message() {
    let tickets = [
        mock_ticket(1, 2, "Prague", "Brno"),
        mock_ticket(2, 2, "Brno", "Ostrava"),
    ];

    let ticket_refs: Vec<&DomainTicket> = tickets.iter().collect();
    let message = TelegramAdapter::format_tickets_message(&ticket_refs);

    assert!(message.contains("Upcoming Tickets"));
    assert!(message.contains("(2)"));
    assert!(message.contains("Prague"));
    assert!(message.contains("Brno"));
    assert!(message.contains("Ostrava"));
}

#[tokio::test]
async fn test_credentials_store_flow() {
    // Use temp file for testing
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join(format!(
        "rustjet_integration_test_{}.json",
        rand::random::<u32>()
    ));
    let test_path = test_file.to_str().unwrap();
    let store = JsonCredentialsStorage::new(test_path).expect("Failed to create store");

    let user_id = 123456;
    let creds = UserCredentials {
        account_code: "TEST_ACCOUNT".to_string(),
        password: "test_password".to_string(),
    };

    // Initially no credentials
    assert!(!store.has(user_id).unwrap());
    assert!(store.get(user_id).unwrap().is_none());

    // Store credentials
    store
        .store(user_id, &creds)
        .expect("Failed to store credentials");

    // Verify stored
    assert!(store.has(user_id).unwrap());

    let retrieved = store.get(user_id).unwrap().expect("Credentials not found");

    assert_eq!(retrieved.account_code, creds.account_code);
    assert_eq!(retrieved.password, creds.password);

    // Update credentials
    let new_creds = UserCredentials {
        account_code: creds.account_code.clone(),
        password: "new_password".to_string(),
    };
    store
        .store(user_id, &new_creds)
        .expect("Failed to update credentials");

    let updated = store.get(user_id).unwrap().unwrap();
    assert_eq!(updated.password, new_creds.password);

    // Delete credentials
    store.delete(user_id).expect("Failed to delete credentials");
    assert!(!store.has(user_id).unwrap());

    // Cleanup
    std::fs::remove_file(&test_path).ok();
}

#[tokio::test]
async fn test_empty_tickets_message() {
    let empty: &[&DomainTicket] = &[];
    let message = TelegramAdapter::format_tickets_message(empty);
    assert_eq!(message, "No upcoming tickets found.");
}
