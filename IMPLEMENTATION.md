# Implementation Guide

## Status: Phase 14 Complete

**Phase 6**: Bot infrastructure
**Phase 7**: Integration testing
**Phase 8**: Documentation
**Phase 9**: Workspace refactoring (4 crates)
**Phase 10**: Storage (SQLite → JSON file)
**Phase 11**: Hexagonal architecture (ports/adapters, TOML config)
**Phase 12**: Telegram MiniApp backend (Axum, real data integration)
**Phase 13**: MiniApp frontend (TypeScript, /webapp command)
**Phase 14**: Dependency updates

## Key Decisions

**MiniApp Architecture:**
- Backend: Axum web server with Telegram WebApp auth
- Frontend: TypeScript, vanilla JS (no framework)
- Storage: Shared JSON file for credentials + notification settings
- CORS configured for web.telegram.org

**Crates:**
- `regiojet-api` - OpenAPI-generated client
- `rustjet-core` - Domain, ports, adapters
- `rustjet-cli` - Bot binary
- `rustjet-web` - MiniApp backend

## Architecture

Hexagonal (ports/adapters):
- **Domain**: Pure business types
- **Ports**: Traits (TicketRepository, CredentialsStorage, NotificationService, NotificationSettingsStorage)
- **Adapters**: RegioJet API, Telegram, JSON storage, WebApp auth
- **Services**: Ticket filtering logic
- **Bot**: Handlers, dialogue state

## Lessons Learned

- Atomic commits essential - avoid large changesets
- Subagents violate atomicity, implement incrementally by hand
- Run `just check` after each change, not end of phase
- Separate commits: features vs formatting/clippy
- Commit phase transitions before starting next phase (Phase 9-10 mistake required manual splitting)

## Outstanding Issues

### Optional: Deployment Guide

Create DEPLOYMENT.md for production:
- systemd service unit file
- NixOS module configuration
- TOML config file management
- Log rotation (journald integration)
- Security: file permissions for credentials.json, TLS for webhook mode
