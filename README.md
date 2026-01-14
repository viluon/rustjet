# RustJet Telegram Bot

Telegram bot for RegioJet ticket management - view upcoming tickets and receive departure notifications.

## Features

- **Telegram Mini App** - Modern web interface for ticket and account management
- **View upcoming tickets** - Display tickets departing in next 30 days via Telegram or web app
- **Automatic notifications** - Alerts before departure (background service)
- **Secure credential storage** - Encrypted password storage in JSON file
- **Multi-user support** - Each Telegram user has separate credentials
- **Interactive login** - Wizard-based account connection via bot or web app

## Setup

### Prerequisites

```bash
# Enter Nix development environment
nix develop
```

### Configuration

1. Copy config template:
   ```bash
   cp config.toml.example config.toml
   ```

2. Edit `config.toml`:
   ```toml
   [telegram]
   bot_token = "your_bot_token_here"

   [storage]
   credentials_path = "credentials.json"

   [notifications]
   minutes_before = 60
   check_interval_seconds = 300
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
- `/webapp` - Open Telegram Mini App
- `/help` - Command list

## Telegram Mini App

Web interface accessible via `/webapp` command provides:

- **Ticket list** - View all upcoming tickets with route, time, and price
- **Account management** - Add/remove RegioJet credentials via web form
- **Notification settings** - Toggle departure notifications on/off
- **Telegram integration** - Authenticated via Telegram WebApp initData

### Web Server

Runs on port configured in `config.toml`:

```toml
[web]
host = "0.0.0.0"
port = 8080
```

**API Endpoints:**
- `GET /health` - Health check
- `GET /api/user` - Get user info and settings
- `GET /api/tickets` - List upcoming tickets
- `POST /api/credentials` - Save RegioJet account
- `DELETE /api/credentials` - Remove account
- `POST /api/settings/notifications` - Update notification preferences

**Tech Stack:**
- Backend: Axum with Telegram WebApp auth
- Frontend: TypeScript, vanilla JS
- CORS: Configured for `https://web.telegram.org`

## Development

### Commands

- `just fmt` - Format code
- `just check` - Run clippy, fmt check, and tests
- `just build` - Build release binary
- `just test` - Run all tests (22 unit + 4 integration)

### Architecture

```
rustjet-cli (composition root)
  └─> rustjet-core
        ├─> domain/          Pure business types
        ├─> ports/           Trait definitions
        ├─> services/        Business logic
        ├─> adapters/        External implementations
        │     ├─> regiojet.rs      (TicketRepository)
        │     ├─> telegram.rs      (NotificationService)
        │     └─> json_storage.rs  (CredentialsStorage)
        └─> bot/             Orchestration
```

**Key traits (ports):**
- `TicketRepository` - Fetch tickets from external API
- `NotificationService` - Send notifications to users
- `CredentialsStorage` - Store/retrieve user credentials

**Crates:**
- `regiojet-api` - Generated API client from OpenAPI spec
- `rustjet-core` - Domain types, ports, services, adapters
- `rustjet-cli` - CLI binary, dependency wiring
- `rustjet-web` - Axum web server for Telegram Mini App

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
