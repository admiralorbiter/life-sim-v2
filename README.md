# Life Roguelite

A choice-driven life simulation game built with Rust. You start as a middle schooler and progress through 4 life stages, making choices, drawing Life Event cards, and keeping your life stable while pursuing goals.

**Designed for classroom use** — teaches tradeoffs, recovery, and planning through gameplay.

## Quick Start

```bash
# Clone and run
git clone <repo-url>
cd life-sim-v2
cargo run
```

Then open [http://localhost:8080](http://localhost:8080) in your browser.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend / Engine | Rust + Actix-web |
| Frontend | Vanilla HTML + CSS + JS |
| Data | JSON files (serde) |
| RNG | ChaCha8Rng (seedable for classroom use) |

## Project Structure

```
src/
├── main.rs              # Actix-web server
├── engine/              # Game logic
│   ├── game_state.rs    # Player state
│   └── rng.rs           # Seeded RNG
├── models/              # Data structures
│   ├── event.rs         # Event cards
│   ├── action.rs        # Player actions
│   ├── decision.rs      # Phase 2 decisions
│   ├── job.rs           # Jobs
│   └── ending.rs        # Game endings
├── api/                 # REST API routes
│   └── routes.rs
└── data_loader.rs       # JSON deserialization

static/                  # Frontend (served by Actix)
data/                    # Game content (JSON)
docs/                    # Design documents
```

## Development

```bash
cargo run                # Start dev server on localhost:8080
cargo test               # Run all tests
cargo clippy             # Lint
cargo build --release    # Build release binary
```

## Documentation

- [Game Design Document](docs/01-game-design-document.md)
- [Technical Architecture](docs/02-technical-architecture.md)
- [Sprint Plan](docs/03-sprint-plan.md)
- [Content Authoring Guide](docs/04-content-authoring-guide.md)
- [Product Backlog](docs/05-product-backlog.md)
