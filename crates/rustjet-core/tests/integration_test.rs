use chrono::Utc;
use regiojet_api::models::{
    Currency, CustomerActions, Line, PassengersInfo, PriceConditions, Section, Ticket,
    TicketSection, TicketState, VehicleType,
};
use rustjet_core::{
    bot::tickets::{format_tickets_message, get_upcoming_tickets},
    storage::credentials::CredentialsStore,
};

/// Create mock ticket for testing
fn mock_ticket(id: i64, departure_time: &str, from: &str, to: &str) -> Ticket {
    let section = TicketSection {
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
                from: from.to_string(),
                to: to.to_string(),
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
            arrival_time: "2026-01-11T12:00:00Z".to_string(),
            arrival_platform: None,
            carrier_id: None,
            free_seats_count: None,
            notices: None,
            services: None,
            delay: None,
            travel_time: "2h".to_string(),
            estimated_arrival_time: None,
        }),
        fixed_seat_reservation: true,
        selected_seats: vec![],
        vehicles: None,
    };

    Ticket {
        id,
        ticket_code: format!("TEST{}", id),
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
        route_sections: vec![section],
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

#[tokio::test]
async fn test_ticket_filtering() {
    let now = Utc::now();
    let in_2_hours = (now + chrono::Duration::hours(2)).to_rfc3339();
    let in_25_hours = (now + chrono::Duration::hours(25)).to_rfc3339();
    let in_50_hours = (now + chrono::Duration::hours(50)).to_rfc3339();

    let tickets = vec![
        mock_ticket(1, &in_2_hours, "Prague", "Brno"),
        mock_ticket(2, &in_25_hours, "Brno", "Ostrava"),
        mock_ticket(3, &in_50_hours, "Ostrava", "Warsaw"),
    ];

    // Filter for next 24 hours - should get only first ticket
    let upcoming_24h = get_upcoming_tickets(&tickets, 24);
    assert_eq!(upcoming_24h.len(), 1);
    assert_eq!(upcoming_24h[0].id, 1);

    // Filter for next 48 hours - should get first two tickets
    let upcoming_48h = get_upcoming_tickets(&tickets, 48);
    assert_eq!(upcoming_48h.len(), 2);
    assert_eq!(upcoming_48h[0].id, 1);
    assert_eq!(upcoming_48h[1].id, 2);

    // Filter for next 72 hours - should get all three tickets
    let upcoming_72h = get_upcoming_tickets(&tickets, 72);
    assert_eq!(upcoming_72h.len(), 3);
}

#[tokio::test]
async fn test_format_tickets_message() {
    let now = Utc::now();
    let in_2_hours = (now + chrono::Duration::hours(2)).to_rfc3339();

    let tickets = [
        mock_ticket(1, &in_2_hours, "Prague", "Brno"),
        mock_ticket(2, &in_2_hours, "Brno", "Ostrava"),
    ];

    let ticket_refs: Vec<&Ticket> = tickets.iter().collect();
    let message = format_tickets_message(&ticket_refs);

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
    let store = CredentialsStore::new(test_path).expect("Failed to create store");

    let user_id = 123456;
    let account_code = "TEST_ACCOUNT";
    let password = "test_password";

    // Initially no credentials
    assert!(!store.has_credentials(user_id).unwrap());
    assert!(store.get_credentials(user_id).unwrap().is_none());

    // Store credentials
    store
        .store_credentials(user_id, account_code, password)
        .expect("Failed to store credentials");

    // Verify stored
    assert!(store.has_credentials(user_id).unwrap());

    let retrieved = store
        .get_credentials(user_id)
        .unwrap()
        .expect("Credentials not found");

    assert_eq!(retrieved.telegram_user_id, user_id);
    assert_eq!(retrieved.regiojet_account_code, account_code);
    assert_eq!(retrieved.password, password);

    // Update credentials
    let new_password = "new_password";
    store
        .store_credentials(user_id, account_code, new_password)
        .expect("Failed to update credentials");

    let updated = store.get_credentials(user_id).unwrap().unwrap();
    assert_eq!(updated.password, new_password);

    // Delete credentials
    store
        .delete_credentials(user_id)
        .expect("Failed to delete credentials");
    assert!(!store.has_credentials(user_id).unwrap());

    // Cleanup
    std::fs::remove_file(&test_path).ok();
}

#[tokio::test]
async fn test_empty_tickets_message() {
    let empty: &[&Ticket] = &[];
    let message = format_tickets_message(empty);
    assert_eq!(message, "No upcoming tickets found.");
}
