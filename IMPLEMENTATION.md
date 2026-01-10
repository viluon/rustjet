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

**Setup:** Use Nix develop, configure .env from .env.example with bot token, run bot binary.

**Bot Commands:**
- /start - Initialize bot
- /login - Link RegioJet account (interactive wizard)
- /mytickets - Show tickets departing in next 30 days
- /notifications - Info about notification settings
- /help - Command list

**Development:** Use `just fmt`, `just check`, `just build`, `just test` commands.

**Architecture:** Telegram → handlers → tickets → APIs; handlers ↔ state; tickets → credentials DB.

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

**Crates:** regiojet-api (generated client), rustjet-core (bot logic), rustjet-cli (binaries).

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

**Port/adapter pattern:** Define domain traits, implement as adapters. Decouple business logic from external APIs.

**Benefits:**
- Domain logic independent of external APIs
- Easy to mock for testing
- Swap implementations without changing business logic

### Replace SQLite with simpler storage

SQLite overkill for simple key-value store. Use serde_json or toml file storage with atomic writes instead.

### Explicit config injection

Load config from file and pass explicitly to services. Remove scattered `dotenv::dotenv()` calls. Enable testing without env vars.
