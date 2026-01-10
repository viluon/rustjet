# Implementation Guide

## Status: Phase 6 Complete ✓

Bot infrastructure implemented with atomic commits:
- Storage layer with SQLite credentials store (base64 placeholder encryption)
- Bot config loading from env vars (.env.example provided)
- Ticket fetching and formatting (RegioJet API integration)
- Command handlers (/start, /login, /mytickets, /notifications, /help)
- Background notification service (periodic ticket checks)
- Conversation state management (login wizard)
- Bot entry point with teloxide dispatcher

## Lessons Learned

Atomic commits essential for clean history. Spawning subagents creates large changesets that violate atomicity - better to implement incrementally by hand. Always `just check` after every change, not just at end of phase. Formatting and clippy fixes belong in separate commits from feature work.

## Phase 7: Integration Testing

Unit tests exist in modules (10 passing tests in bot/tickets and storage/credentials). Need integration tests for end-to-end flows.

### Add dev-dependencies
```toml
[dev-dependencies]
mockito = "1.5"      # HTTP mocking for RegioJet API
tokio-test = "0.4"   # Async test utilities
```

### Create tests/integration_test.rs

Mock RegioJet API endpoints:
- POST /login_registered_account → return mock token
- GET /tickets → return mock ticket list with various states

Test flows:
1. Login flow: verify credentials → store in DB → fetch tickets
2. Ticket filtering: mock tickets with different departure times → verify get_upcoming_tickets filtering
3. Notification service: mock user with upcoming departure → verify bot sends message
4. Error handling: invalid credentials → appropriate error message

Use mockito to create HTTP server stub. Test with in-memory SQLite (":memory:"). Mock teloxide Bot with Arc<Mutex<Vec<Message>>> to capture sent messages.

Commit: "Add integration tests for login and ticket flows"

## Phase 8: Documentation

### Update README.md

Current README is minimal. Replace with comprehensive guide:

**Features section:**
- View upcoming RegioJet tickets via Telegram
- Automatic notifications before departure
- Secure credential storage (encrypted)
- Multi-user support

**Setup:**
```bash
# With Nix
nix develop
just check

# Configure bot
cp .env.example .env
# Edit .env with bot token from @BotFather

# Run
cargo run --bin bot
```

**Bot Commands:**
- /start - Initialize bot
- /login - Link RegioJet account (interactive wizard)
- /mytickets - Show tickets departing in next 30 days
- /notifications - Info about notification settings
- /help - Command list

**Development:**
```bash
just fmt      # Format code
just check    # Format + clippy + test
just build    # Build release binary
just test     # Run tests
```

**Architecture:**
```
┌─────────────┐
│ Telegram    │
│ Bot API     │
└──────┬──────┘
       │
       v
┌──────────────────┐      ┌──────────────┐
│ bot/handlers.rs  │─────>│ bot/state.rs │
│ (commands)       │      │ (dialogue)   │
└────┬─────────────┘      └──────────────┘
     │
     v
┌──────────────────┐      ┌──────────────────┐
│ bot/tickets.rs   │─────>│ apis/*           │
│ (fetch/format)   │      │ (RegioJet client)│
└──────────────────┘      └──────────────────┘
     │
     v
┌──────────────────────┐  ┌────────────────────┐
│ storage/credentials  │─>│ credentials.db     │
│ (encrypted storage)  │  │ (SQLite)           │
└──────────────────────┘  └────────────────────┘
```

Commit: "Document setup, usage, and architecture"

### Optional: DEPLOYMENT.md

Create deployment guide for production:
- systemd service unit file
- NixOS module configuration
- Environment variable management (systemd EnvironmentFile)
- Log rotation (journald integration)
- Security: file permissions for credentials.db, TLS for webhook mode

Commit: "Add deployment guide"

## Phase 9: Workspace Refactoring

### Move OpenAPI generated code to separate crate

Current: All code in monolithic rustjet crate.
Problem: Generated API code pollutes main crate, hard to regenerate, couples business logic to API client.

Solution: Cargo workspace with multiple crates.

**Directory structure:**
```
rustjet/
├── Cargo.toml          # Workspace manifest
├── crates/
│   ├── regiojet-api/   # Generated OpenAPI client
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── apis/
│   │       └── models/
│   ├── rustjet-core/   # Bot implementation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── bot/
│   │       ├── storage/
│   │       └── lib.rs
│   └── rustjet-cli/    # CLI tools
│       ├── Cargo.toml
│       └── src/
│           ├── bin/bot.rs
│           └── bin/main.rs
```

**Workspace Cargo.toml:**
```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.dependencies]
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
serde = "1.0"
# ... shared deps
```

**Benefits:**
- Clean separation: API client vs business logic
- Independent versioning
- Easier to regenerate API code without conflicts
- Smaller compile units

**Migration steps:**
1. Create workspace structure with `cargo new --lib` for each crate
2. Move src/apis + src/models → regiojet-api crate
3. Move src/bot + src/storage → rustjet-core crate
4. Move src/bin → rustjet-cli crate
5. Update dependencies: rustjet-core depends on regiojet-api
6. Update imports: `crate::apis` → `regiojet_api::apis`
7. Test and commit atomically per crate

Each crate commit separately: "Create regiojet-api workspace crate", "Create rustjet-core workspace crate", etc.

## Phase 10: Architectural Improvements

### Adopt hexagonal architecture

Current issues:
- bot/tickets.rs directly imports generated API types
- Storage layer tightly coupled to SQLite
- Config loaded from env inside modules

**Port/adapter pattern:**

Define traits in core domain:
```rust
// rustjet-core/src/domain/ports.rs
trait TicketRepository {
    async fn fetch_tickets(&self, user: &User) -> Result<Vec<Ticket>>;
}

trait CredentialStore {
    fn save(&self, creds: Credentials) -> Result<()>;
    fn load(&self, user_id: i64) -> Result<Option<Credentials>>;
}
```

Implementations as adapters:
```rust
// rustjet-core/src/adapters/regiojet.rs
struct RegioJetTicketRepository { client: RegioJetClient }

impl TicketRepository for RegioJetTicketRepository {
    async fn fetch_tickets(&self, user: &User) -> Result<Vec<Ticket>> {
        // Translate domain types to/from API types
    }
}
```

**Benefits:**
- Domain logic independent of external APIs
- Easy to mock for testing
- Swap implementations without changing business logic

### Replace SQLite with simpler storage

SQLite overkill for simple key-value store. Replace with:
```rust
// rustjet-core/src/adapters/file_store.rs
#[derive(Serialize, Deserialize)]
struct CredentialStorage {
    users: HashMap<i64, Credentials>,
}

impl CredentialStore for FileCredentialStore {
    fn save(&self, creds: Credentials) -> Result<()> {
        let mut storage = self.load_from_disk()?;
        storage.users.insert(creds.user_id, creds);
        self.save_to_disk(&storage)
    }
}
```

Use serde_json or toml for serialization. Atomic file writes with temp file + rename.

### Explicit config injection

Remove `dotenv::dotenv()` calls scattered in code. Instead:

```rust
// rustjet-cli/src/bin/bot.rs
fn main() -> Result<()> {
    let config = Config::from_file("config.toml")?;
    let bot = BotService::new(config);
    bot.run().await
}
```

Config becomes dependency injected, not global state. Testable without env vars.
