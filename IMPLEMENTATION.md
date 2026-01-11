# Implementation Guide

## Status: Phase 10 Complete ✓

**Phase 6**: Bot infrastructure implemented with atomic commits.

**Phase 7**: Integration testing complete - 14 tests passing (10 unit + 4 integration).

**Phase 8**: Documentation complete - comprehensive README with setup, commands, architecture.

**Phase 9**: Workspace refactoring complete - 3 crates (regiojet-api, rustjet-core, rustjet-cli).

**Phase 10**: Storage simplified - replaced SQLite with JSON file storage, atomic writes.

## Lessons Learned

Atomic commits essential for clean history. Spawning subagents creates large changesets that violate atomicity - better to implement incrementally by hand. Always `just check` after every change, not just at end of phase. Formatting and clippy fixes belong in separate commits from feature work.

**Phase 9-10 mistake**: Implemented both phases without intermediate commits. Required `git reset --soft HEAD~` and manual splitting into 5 commits. Should have committed Phase 9 workspace refactoring before starting Phase 10 storage changes.

## Outstanding Issues

### Reliance on environment variables
- Remove dotenv dependency
- Load config from a file using serde instead

### Architecture

Current issues:
- bot/tickets.rs directly imports generated API types (tight coupling to API client)
- Config loaded from env inside modules (scattered `dotenv::dotenv()` calls)

**Planned improvements:**
- Adopt hexagonal architecture with port/adapter pattern
- Define domain traits, implement as adapters
- Decouple business logic from external APIs
- Explicit config injection - load config from file and pass to services
- Enable testing without env vars

**Benefits:**
- Domain logic independent of external APIs
- Easy to mock for testing
- Swap implementations without changing business logic

### Optional: Deployment Guide

Create DEPLOYMENT.md for production:
- systemd service unit file
- NixOS module configuration
- Environment variable management (systemd EnvironmentFile)
- Log rotation (journald integration)
- Security: file permissions for credentials.json, TLS for webhook mode
