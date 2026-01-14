# Implementation Guide

## Status: Phase 13 Complete - Telegram MiniApp Fully Operational

**Phase 6**: Bot infrastructure implemented with atomic commits.

**Phase 7**: Integration testing complete.

**Phase 8**: Documentation complete - comprehensive README with setup, commands, architecture.

**Phase 9**: Workspace refactoring complete - 3 crates (regiojet-api, rustjet-core, rustjet-cli).

**Phase 10**: Storage simplified - replaced SQLite with JSON file storage, atomic writes.

**Phase 11**: Hexagonal architecture complete - ports/adapters pattern, TOML config, domain types.

**Phase 12**: Telegram MiniApp backend - complete (all 22 commits).

**Phase 13**: Telegram MiniApp frontend & integration - complete (4 commits)

## Phase 12: Telegram MiniApp Implementation

### Completed (22/22 commits)

**Phase 1: Config & Infrastructure (6 commits)**
1. Add web server config to TOML structure
2. Configure web server host and port
3. Add AppState for dependency injection
4. Wire config and adapters into web server
5. Validate Telegram WebApp auth signature
6. Add integration tests for health and auth

**Phase 2: NotificationSettings Port & Adapter (6 commits)**
7. Add NotificationSettings domain type
8. Define NotificationSettingsStorage port
9. Extend JSON schema for notification settings
10. Implement notification settings in JSON adapter
11. Wire notification settings storage to AppState
12. Add tests for notification settings storage

**Phase 3: Real Data & Existing Endpoints (5 commits)**
13. Add JSON serialization to domain types
14. Return real tickets from RegioJet API
15. Return real user status from storage
16. Add structured error handling for API
17. Add integration tests for user and tickets

**Phase 4: New Endpoints (5 commits)**
18. Add POST /api/credentials endpoint
19. Add DELETE /api/credentials endpoint
20. Add POST /api/settings/notifications endpoint
21. Add CORS middleware
22. Add integration tests for Phase 4

### Backend API Complete

All endpoints implemented with real data integration:
- GET /health
- GET /api/tickets (real RegioJet API data)
- GET /api/user (credentials & notification status)
- POST /api/credentials (store RegioJet account)
- DELETE /api/credentials (remove stored account)
- POST /api/settings/notifications (toggle notifications)

## Phase 13: Frontend & Bot Integration

### Completed (4 commits)

1. Add API client to frontend
2. Implement complete frontend UI (tickets, credentials form, notification toggle)
3. Add /webapp command (inline keyboard with WebApp button)
4. Update README with Telegram Mini App docs

### Features Delivered

**Frontend:**
- TypeScript API client with fetch wrapper
- Ticket list component with route, time, price display
- Credentials form for RegioJet account management
- Notification toggle with instant API updates
- Remove account functionality
- Telegram theme integration (colors, styling)

**Bot Integration:**
- `/webapp` command opens Mini App via inline keyboard
- WebApp button launches authenticated web interface

**Documentation:**
- Updated README with Mini App features
- API endpoint documentation
- Web server configuration guide
- Tech stack overview

### Architecture Updates

The web server uses:
- **Framework**: Axum with State-based dependency injection
- **Authentication**: Telegram WebApp signature validation
- **Storage**: JSON file-based (credentials, notification settings)
- **API Integration**: Real RegioJet API for ticket fetching
- **Error Handling**: Structured errors with proper HTTP status codes
- **Testing**: Integration tests for health, auth, user data, and tickets

## Architecture

```
rustjet-cli (composition root)
  └─> rustjet-core
        ├─> domain/          Pure business types (DomainTicket, UserCredentials)
        ├─> ports/           Trait definitions (TicketRepository, CredentialsStorage, NotificationService)
        ├─> services/        Business logic (TicketService)
        ├─> adapters/        External implementations
        │     ├─> regiojet.rs      (TicketRepository impl)
        │     ├─> telegram.rs      (NotificationService impl)
        │     └─> json_storage.rs  (CredentialsStorage impl)
        └─> bot/             Orchestration (handlers, notifications, state)
```

## Lessons Learned

Atomic commits essential for clean history. Spawning subagents creates large changesets that violate atomicity - better to implement incrementally by hand. Always `just check` after every change, not just at end of phase. Formatting and clippy fixes belong in separate commits from feature work.

**Phase 9-10 mistake**: Implemented both phases without intermediate commits. Required `git reset --soft HEAD~` and manual splitting into 5 commits. Should have committed Phase 9 workspace refactoring before starting Phase 10 storage changes.

## Outstanding Issues

### Optional: Deployment Guide

Create DEPLOYMENT.md for production:
- systemd service unit file
- NixOS module configuration
- TOML config file management
- Log rotation (journald integration)
- Security: file permissions for credentials.json, TLS for webhook mode
