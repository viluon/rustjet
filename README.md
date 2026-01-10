# RustJet Telegram Bot

Telegram bot for RegioJet ticket management - view upcoming tickets and receive departure notifications.

## Features

- **View upcoming tickets** - Display tickets departing in next 30 days via Telegram
- **Automatic notifications** - Alerts before departure (background service)
- **Secure credential storage** - Encrypted password storage with SQLite
- **Multi-user support** - Each Telegram user has separate credentials
- **Interactive login** - Wizard-based account connection

## Setup

### Prerequisites

```bash
# Enter Nix development environment
nix develop
```

### Configuration

1. Copy environment template:
   ```bash
   cp .env.example .env
   ```

2. Configure `.env`:
   ```
   TELOXIDE_TOKEN=your_bot_token_here
   RUST_LOG=info
   ```

   Get bot token from [@BotFather](https://t.me/botfather) on Telegram.

### Run

```bash
just run
```

## Bot Commands

- `/start` - Initialize bot
- `/login` - Link RegioJet account (interactive wizard prompts for account code + password)
- `/mytickets` - Show tickets departing in next 30 days
- `/notifications` - Info about notification settings
- `/help` - Command list

## Development

### Commands

- `just fmt` - Format code
- `just check` - Run clippy, fmt check, and tests
- `just build` - Build release binary
- `just test` - Run all tests (14 tests: 10 unit + 4 integration)

### Architecture

```
Telegram Bot (teloxide)
    |
    v
Handlers (src/bot/handlers.rs)
    |
    +---> Tickets (src/bot/tickets.rs)
    |         |
    |         +---> RegioJet API (src/apis/)
    |
    +---> Credentials Store (src/storage/credentials.rs)
              |
              +---> SQLite Database
```

**Data flow:**
- User commands → handlers → business logic
- Handlers ↔ conversation state (login wizard)
- Tickets module → RegioJet API client (generated from OpenAPI)
- Credentials store → SQLite with base64 encoding

**Key modules:**
- `src/bot/handlers.rs` - Command handlers and login wizard
- `src/bot/tickets.rs` - Ticket fetching and formatting
- `src/bot/state.rs` - Conversation state management
- `src/bot/notifications.rs` - Background notification service
- `src/storage/credentials.rs` - Encrypted credential storage
- `src/apis/` - Generated RegioJet API client
- `src/models/` - Generated API models

## Testing

Unit tests in module files, integration tests in `tests/`:
- Ticket filtering by departure time
- Message formatting (Telegram MarkdownV2)
- Credentials storage (create, read, update, delete)

```bash
just test
```

## License

See LICENSE file.
