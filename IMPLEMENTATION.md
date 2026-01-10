# Implementation Guide - Phases 6-8

## Phase 6: Telegram Bot

### Directory structure
```
src/
├── bin/
│   ├── main.rs          # Existing CLI PoC
│   └── bot.rs           # New: Bot entry point
├── bot/
│   ├── mod.rs
│   ├── config.rs        # Config from env vars
│   ├── handlers.rs      # Command handlers
│   ├── tickets.rs       # Ticket viewing logic
│   ├── notifications.rs # Background notification service
│   └── state.rs         # Conversation state
├── storage/
│   ├── mod.rs
│   └── credentials.rs   # User credential storage
```

### Dependencies to add
```toml
rusqlite = { version = "0.32", features = ["bundled"] }
dotenv = "0.15"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### Configuration (bot/config.rs)
Environment variables:
- TELEGRAM_BOT_TOKEN - from @BotFather
- CREDENTIALS_DB_PATH - SQLite database path (default: credentials.db)
- NOTIFICATION_MINUTES_BEFORE - when to notify (default: 60)
- NOTIFICATION_CHECK_INTERVAL - polling interval (default: 300s)

Create .env.example with these vars.

### Storage (storage/credentials.rs)
SQLite schema:
```sql
CREATE TABLE credentials (
    telegram_user_id INTEGER PRIMARY KEY,
    regiojet_account_code TEXT NOT NULL,
    encrypted_password TEXT NOT NULL
)
```

Functions:
- CredentialsStore::new(db_path) - Initialize DB
- store_credentials(user_id, code, password) - Save encrypted
- get_credentials(user_id) - Retrieve and decrypt

Note: Use placeholder encryption initially, improve later.

### Bot Commands (bot/handlers.rs)
Implement:
- /start - Welcome message
- /login - Credential setup wizard (multi-step conversation)
- /mytickets - View upcoming tickets
- /notifications - Toggle notifications
- /help - Show commands

Use teloxide BotCommands derive macro.

### Ticket Viewing (bot/tickets.rs)
Core functions:
- fetch_user_tickets(account_code, password) - Login + fetch from RegioJet API
- format_tickets_message(tickets) - Format for Telegram
- get_upcoming_tickets(tickets, hours) - Filter by time

Integration:
- Use login_registered_account() from generated API
- Use get_all_tickets() from generated API
- Parse ticket sections (from → to, time, price)

### Notifications (bot/notifications.rs)
Background service:
- Runs every NOTIFICATION_CHECK_INTERVAL seconds
- For each user with stored credentials:
  1. Fetch their tickets
  2. Check if any depart within notification window
  3. Send Telegram notification if needed

Functions:
- start_notification_service(bot, store, config) - Main loop
- check_and_send_notifications() - Check all users
- send_departure_notification(bot, user_id, info) - Send message

### Bot Entry Point (bin/bot.rs)
1. Load config from env
2. Initialize credentials store
3. Create bot instance
4. Spawn notification service in background
5. Setup message dispatcher with handlers
6. Start polling for updates

Add to Cargo.toml:
```toml
[[bin]]
name = "bot"
path = "src/bin/bot.rs"
```

Update .gitignore:
- .env
- credentials.db

## Phase 7: Testing

### Unit tests
Test in respective modules:
- format_tickets_message() with mock tickets
- Upcoming ticket filtering logic
- Credential encryption/decryption

### Integration tests
Create tests/integration_test.rs:
- Mock RegioJet API with mockito crate
- Test full login → fetch tickets flow
- Test bot handlers with mock responses

Add to Cargo.toml dev-dependencies:
```toml
[dev-dependencies]
tokio-test = "0.4"
mockito = "1.5"
```

## Phase 8: Documentation

### Update README.md
Add:
- Features overview (view tickets, notifications)
- Setup with Nix
- Configuration guide (environment variables)
- Bot commands list
- Development workflow (just commands)
- Architecture diagram (text-based)

### Create .env.example
Document all environment variables with example values.

### Update .gitignore
Ensure .env and credentials.db are excluded.

### Optional: DEPLOYMENT.md
Add deployment guides for:
- systemd service
- NixOS module
- Environment variable management