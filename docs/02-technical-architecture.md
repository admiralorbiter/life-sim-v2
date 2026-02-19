# Life Roguelite — Technical Architecture

## 1. Tech Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------| 
| **Backend / Engine** | Rust (Actix-web) | Type-safe game engine, serves the frontend, handles all logic |
| **Frontend** | Vanilla HTML + CSS + JavaScript | Simple, no build step, served as static files by the Rust binary |
| **Serialization** | serde + serde_json | Deserialize JSON data files into Rust structs |
| **RNG** | rand + rand_chacha | Seedable, deterministic RNG for classroom reproducibility |
| **Asset Embedding** | rust-embed | Embed `static/` and `data/` into the release binary |
| **Testing** | cargo test | Unit + integration tests for the game engine |
| **Deployment** | Single binary | `cargo run` for dev; `cargo build --release` for distribution |

No external runtime is required. `cargo run` compiles and starts a local web server. The browser opens to `http://localhost:8080`.

---

## 2. Project Structure

```
life-roguelite/
├── Cargo.toml                    # Dependencies: actix-web, serde, rand, rust-embed
├── src/
│   ├── main.rs                   # Actix-web server, routes, static file serving
│   ├── engine/
│   │   ├── mod.rs
│   │   ├── game_state.rs         # GameState struct + initialization
│   │   ├── turn_runner.rs        # Executes one full turn (4 phases)
│   │   ├── event_deck.rs         # Card draw logic, filtering by stage/rarity
│   │   ├── stat_calculator.rs    # Applies stat changes, clamps, triggers
│   │   ├── credential_system.rs  # Tag management, job matching
│   │   ├── ending_resolver.rs    # Evaluates final state → ending
│   │   └── rng.rs                # Seeded RNG wrapper (ChaCha8Rng)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── event.rs              # EventCard, EventOption, StatEffect structs
│   │   ├── action.rs             # Action struct
│   │   ├── decision.rs           # Decision struct
│   │   ├── job.rs                # Job struct
│   │   └── ending.rs             # Ending struct
│   ├── api/
│   │   ├── mod.rs
│   │   └── routes.rs             # REST endpoints: new_game, submit_action, get_state, etc.
│   └── data_loader.rs            # Load and parse JSON data files
├── static/
│   ├── index.html                # Single-page game UI
│   ├── css/
│   │   └── style.css             # All styling
│   └── js/
│       ├── app.js                # Main game controller — fetches state, renders phases
│       ├── components.js         # UI component renderers (stats bar, event card, etc.)
│       └── api.js                # Fetch wrappers for REST API calls
├── data/
│   ├── events.json               # All 40 event cards
│   ├── actions.json              # Action definitions per stage
│   ├── decisions.json            # Decision options per stage
│   ├── jobs.json                 # Job definitions with tag requirements
│   ├── endings.json              # Ending conditions + text
│   └── kc-config.json            # KC-specific flavor data
└── tests/
    ├── engine_tests.rs           # Integration tests for the game engine
    └── api_tests.rs              # HTTP endpoint tests
```

---

## 3. Core Data Models

### Game State

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub current_stage: Stage,
    pub current_turn: u32,
    pub total_turns: u32,

    // Resources
    pub money: i32,
    pub stress: i32,
    pub support: i32,
    pub time_slots: u32,
    pub credentials: Vec<String>,

    // Tracking
    pub current_job: Option<Job>,
    pub monthly_bills: i32,        // introduced in Stage D
    pub emergency_fund: i32,       // introduced in Stage D
    pub decision_log: Vec<DecisionEntry>,
    pub used_event_ids: Vec<String>,

    // Meta
    pub seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Stage {
    MiddleSchool,
    HighSchool,
    PostHigh,
    EarlyAdult,
}
```

### Event Card

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCard {
    pub id: String,
    pub title: String,
    pub flavor_text: String,
    pub stages: Vec<Stage>,
    pub rarity: Rarity,
    pub options: Vec<EventOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOption {
    pub label: String,
    pub description: String,
    pub effects: Vec<StatEffect>,
    pub delayed_effects: Option<Vec<DelayedEffect>>,
    pub requires_support: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatEffect {
    pub stat: StatType,
    pub delta: i32,
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatType {
    Money,
    Stress,
    Support,
    TimeSlots,
    Credentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}
```

### Job

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub required_tags: Vec<String>,
    pub recommended_tags: Vec<String>,
    pub pay_per_turn: i32,
    pub stress_per_turn: i32,
    pub growth_rate: u32,
    pub stages: Vec<Stage>,
}
```

---

## 4. REST API

All game logic runs server-side. The frontend is a thin UI that calls these endpoints.

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/new_game` | Create a new game (optional `seed` and `scenario` params). Returns `GameState`. |
| `GET`  | `/api/state` | Get current game state. |
| `POST` | `/api/submit_actions` | Submit Phase 1 action selections. Returns updated state. |
| `POST` | `/api/submit_decision` | Submit Phase 2 decision. Returns updated state. |
| `GET`  | `/api/draw_event` | Draw the Phase 3 event card. Returns `EventCard`. |
| `POST` | `/api/submit_event_response` | Submit Phase 3 event response. Returns updated state + feedback. |
| `POST` | `/api/advance_turn` | Advance to next turn / stage. Returns updated state. |
| `GET`  | `/api/endings` | Get the resolved ending after the final turn. |
| `GET`  | `/api/timeline` | Get the decision timeline recap. |

Game state is held in server memory (one game per process for MVP). No database needed.

---

## 5. Key Engine Logic

### Turn Runner (Pseudocode)

```rust
pub fn run_turn(state: &mut GameState, choices: &PlayerChoices, data: &GameData) {
    // Phase 1: Apply action selections
    for action_id in &choices.action_ids {
        let action = data.get_action(action_id);
        stat_calculator::apply_effects(state, &action.effects);
    }

    // Phase 2: Apply decision
    let decision_option = data.get_decision_option(&choices.decision_id, choices.option_index);
    stat_calculator::apply_effects(state, &decision_option.effects);
    if let Some(tag) = &decision_option.grants_tag {
        state.credentials.push(tag.clone());
    }

    // Phase 3: Event resolved via API (player picks response in browser)
    // Event effects applied in submit_event_response handler

    // Phase 4: Feedback
    stat_calculator::apply_delayed_effects(state);
    stat_calculator::apply_job_income(state);
    stat_calculator::apply_monthly_bills(state);  // Stage D only
    stat_calculator::check_stress_threshold(state);
    state.current_turn += 1;

    if is_stage_end(state) {
        transition_stage(state);
    }
}
```

### Seeded RNG

Uses `rand_chacha::ChaCha8Rng` with a seed derived from the classroom seed string:

```rust
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub fn create_rng(seed_str: &str) -> ChaCha8Rng {
    let mut seed_bytes = [0u8; 32];
    let bytes = seed_str.as_bytes();
    for (i, &b) in bytes.iter().enumerate().take(32) {
        seed_bytes[i] = b;
    }
    ChaCha8Rng::from_seed(seed_bytes)
}
```

---

## 6. UI Layout (Single Screen)

```
┌──────────────────────────────────────────┐
│  Stage: High School    Turn: 3 / 6       │
├──────────────────────────────────────────┤
│  💰 $340   😰 42/100   🤝 6/10          │
│  📚 [Dual Credit] [Customer Service]     │
├──────────────────────────────────────────┤
│                                          │
│   [ CURRENT PHASE CONTENT ]              │
│                                          │
│   - Action picker (Phase 1)              │
│   - Decision panel (Phase 2)             │
│   - Event card (Phase 3)                 │
│   - Feedback toast (Phase 4)             │
│                                          │
├──────────────────────────────────────────┤
│  [ Continue ]                            │
└──────────────────────────────────────────┘
```

One screen. Stats always visible at top. Center area swaps between phases. Rendered by `static/js/components.js`, driven by fetch calls to the REST API.

---

## 7. Data Pipeline

All game-balancing data is authored in JSON and loaded at startup:

```
data/events.json     →  data_loader.rs  →  EventCard structs   →  event_deck.rs
data/actions.json    →  data_loader.rs  →  Action structs      →  turn_runner.rs
data/decisions.json  →  data_loader.rs  →  Decision structs    →  turn_runner.rs
data/jobs.json       →  data_loader.rs  →  Job structs         →  credential_system.rs
data/kc-config.json  →  data_loader.rs  →  flavor text, costs, industry names
```

To rebalance: edit JSON, restart `cargo run`. No code changes needed.

In release builds, `rust-embed` bakes all `data/` and `static/` files into the binary for zero-dependency distribution.

---

## 8. Accessibility & Deployment

- All interactions keyboard-navigable
- Color is never the sole indicator (icons + labels on stats)
- Responsive: playable on Chromebook screens (1366×768 minimum)
- `cargo run` serves on `localhost:8080` — no login, no accounts, no data collection
- Optional: `?seed=ABCD1234` query param for classroom use
- `cargo build --release` produces a single distributable binary
