# Implementation Guide

## Status: Phase 11 Complete ✓

**Phase 6**: Bot infrastructure implemented with atomic commits.

**Phase 7**: Integration testing complete.

**Phase 8**: Documentation complete - comprehensive README with setup, commands, architecture.

**Phase 9**: Workspace refactoring complete - 3 crates (regiojet-api, rustjet-core, rustjet-cli).

**Phase 10**: Storage simplified - replaced SQLite with JSON file storage, atomic writes.

**Phase 11**: Hexagonal architecture complete - ports/adapters pattern, TOML config, domain types.

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
