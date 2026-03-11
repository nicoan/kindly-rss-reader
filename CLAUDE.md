# CLAUDE.md

## Project Overview

Kindly RSS Reader is a self-hosted feed aggregator (RSS and Atom) built in Rust, optimized for e-ink devices (Kindle) and low-end hardware (Raspberry Pi).

## Build & Run

```bash
cargo build                # Debug build
cargo build --release      # Release build
cargo run                  # Run locally (default: http://0.0.0.0:3000)
cargo test                 # Run tests
cargo check                # Type-check without building
```

Docker multi-arch builds are available via the Makefile:
```bash
make docker-build          # Build for x86_64, arm64v8, armv7, armv6
```

## Architecture

Layered architecture with clear separation:

- **controllers/** - HTTP request handlers (Axum)
- **services/** - Business logic and orchestration
- **repositories/** - Data access (SQLite)
- **providers/** - External integrations (feed parsing, HTML processing, favicon, images)
- **models/** - Domain models (Feed, Article, ParsedFeed)
- **view_models/** - Data structures for template rendering
- **middlewares/** - HTTP middleware (error handling)
- **templates/** - MiniJinja HTML templates
- **static/** - CSS, fonts, images
- **migrations/** - SQL migration files

## Tech Stack

- **Language:** Rust (Edition 2021)
- **Web Framework:** Axum 0.7
- **Database:** SQLite
- **Async Runtime:** Tokio
- **Templating:** MiniJinja
- **Feed Parsing:** `rss` + `atom_syndication`
- **HTML Processing:** Scraper

## Key Conventions

- **Naming:** Traits are named without suffix (`FeedService`), implementations use `Impl` suffix (`FeedServiceImpl`)
- **Error handling:** Custom error types via `thiserror`; `anyhow` for fallible operations
- **Dependency injection:** Trait-based with generic type parameters
- **Database:** UUIDs as primary keys, UTC timestamps, foreign keys with cascading deletes
- **Templates:** Template name constants (e.g., `TEMPLATE_NAME_FEED_LIST`), context objects passed to MiniJinja
- **Modules:** Each feature area has its own `mod.rs` aggregating public items, with separate `error.rs` files

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `IP` | `0.0.0.0` | Bind IP address |
| `PORT` | `3000` | Bind port |
| `DATA_PATH` | `.` | Data directory (database, articles, favicons) |
| `STATIC_DATA_PATH` | `.` | Static assets path (migrations, templates, static) |
| `MAX_ARTICLES_QTY_TO_DOWNLOAD` | `0` (all) | Max articles to fetch when adding a feed |
| `RUST_LOG` | `INFO` | Log level (TRACE/DEBUG/INFO/WARN/ERROR) |
| `minutes_to_check_for_updates` | `120` | Feed update check interval in minutes |
