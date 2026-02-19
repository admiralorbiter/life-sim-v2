# Life Roguelite — Sprint Plan

**Sprint duration:** 2 weeks each
**Total estimated timeline:** 10 sprints (~20 weeks)
**Approach:** Playable slice first, then expand content and polish.

---

## Sprint 0: Project Setup (Week 1)

**Goal:** Dev environment ready, architecture scaffolded.

### Tasks
- [ ] Initialize Rust project: `cargo init`
- [ ] Add dependencies to `Cargo.toml`: actix-web, serde, serde_json, rand, rand_chacha, rust-embed
- [ ] Create project folder structure per architecture doc (`src/engine/`, `src/models/`, `src/api/`, `static/`, `data/`)
- [ ] Implement `main.rs` — Actix-web server serving static files from `static/`
- [ ] Create placeholder JSON data files (events, actions, decisions, jobs, endings) in `data/`
- [ ] Implement `data_loader.rs` — load and deserialize all JSON files at startup
- [ ] Implement seeded RNG wrapper (`engine/rng.rs`) using `rand_chacha::ChaCha8Rng`
- [ ] Define all model structs in `src/models/` with serde derives
- [ ] Create initial `static/index.html` with placeholder UI
- [ ] Write `README.md` with `cargo run` instructions
- [ ] Set up GitHub Actions CI: `cargo test` + `cargo clippy` on push

### Definition of Done
- `cargo run` serves a placeholder page at `http://localhost:8080`
- JSON data files deserialize into Rust structs without errors
- Seeded RNG produces deterministic sequences given a seed
- CI pipeline runs `cargo test` on push

---

## Sprint 1: Core Engine — Turn Loop (Weeks 2–3)

**Goal:** The engine can run a single turn end-to-end with hardcoded data.

### Tasks
- [ ] Implement `stat_calculator.rs` — apply stat changes with clamping and threshold checks
- [ ] Implement `event_deck.rs` — filter by stage, draw by rarity, track used cards
- [ ] Implement `turn_runner.rs` — orchestrate the 4-phase loop (plan → commit → event → feedback)
- [ ] Define action effects in `data/actions.json` (Stage A only for now)
- [ ] Define 5 test event cards in `data/events.json` (Stage A)
- [ ] Define 2 test decisions in `data/decisions.json` (Stage A)
- [ ] Implement `GameState` initialization with default values
- [ ] Write unit tests for `stat_calculator` (clamping, stress threshold, support bonus)
- [ ] Write unit tests for `event_deck` (filtering, no-repeat draw, rarity weighting)
- [ ] Write integration test: run 3 turns, verify state mutations

### Definition of Done
- `turn_runner` executes a full turn given mock player choices
- Stats clamp correctly (stress 0–100, support 0–10, money ≥ 0 triggers debt)
- Event deck filters by stage and never repeats within a playthrough
- All tests pass via `cargo test`

---

## Sprint 2: Single-Screen UI + REST API (Weeks 4–5)

**Goal:** Playable single-stage prototype — Stage A only, in the browser.

### Tasks
- [ ] Implement REST API endpoints in `src/api/routes.rs`:
  - `POST /api/new_game` — create game with seed
  - `GET /api/state` — return current state as JSON
  - `POST /api/submit_actions` — Phase 1
  - `POST /api/submit_decision` — Phase 2
  - `GET /api/draw_event` — Phase 3 card draw
  - `POST /api/submit_event_response` — Phase 3 response
  - `POST /api/advance_turn` — next turn
- [ ] Build `static/index.html` — single-page game UI layout
- [ ] Build `static/css/style.css` — stats bar, card styling, responsive layout
- [ ] Build `static/js/api.js` — fetch wrappers for all REST endpoints
- [ ] Build `static/js/components.js` — render stats bar, action picker, decision panel, event card, feedback toast
- [ ] Build `static/js/app.js` — game controller: phase state machine, fetches + renders
- [ ] Mobile-responsive layout (single column, stacked phases)

### Definition of Done
- Player can play through Stage A (3–4 turns) in the browser via `cargo run`
- Stats update visibly after each phase
- Event card options are clickable and apply effects
- Playable on 1366×768 screen

---

## Sprint 3: All Stages + Stage Transitions (Weeks 6–7)

**Goal:** Full 12-turn playthrough from Stage A through Stage D.

### Tasks
- [ ] Author actions, decisions, and events for Stages B, C, and D
  - Stage B: 8 event cards, 3 decisions, action pool
  - Stage C: 6 event cards, path selection decision, action pool
  - Stage D: 10 event cards, 3 decisions, action pool + budget system
- [ ] Build stage transition screen in frontend (summary of stage, preview of next)
- [ ] Implement stage progression logic in `turn_runner.rs`
- [ ] Implement monthly bills system (Stage D): deduct from money each turn
- [ ] Implement emergency fund mechanic (Stage D): optional save action
- [ ] Add delayed effects system (effects that trigger N turns after being applied)
- [ ] Update stats bar UI to show bills / emergency fund in Stage D

### Definition of Done
- Player can complete a full 12-turn game across all 4 stages
- Stage transitions display a summary
- Monthly bills deduct correctly in Stage D
- Delayed effects fire on the correct turn

---

## Sprint 4: Credential & Job System (Weeks 8–9)

**Goal:** Tags, jobs, and the alignment mechanic are functional.

### Tasks
- [ ] Implement `credential_system.rs` — add/remove tags, match against job requirements
- [ ] Author `data/jobs.json` — 8–10 jobs across stages with tag requirements
- [ ] Add job selection API endpoint and UI (available in Stages B–D commit phase)
- [ ] Implement job income per turn
- [ ] Implement misalignment penalty (lower pay, +stress if missing recommended tags)
- [ ] Implement growth mechanic (earn a tag after N turns in a growth job)
- [ ] Display credential tags in stats bar with visual indicators
- [ ] Add "job board" view showing available jobs + requirements
- [ ] Write tests for credential matching and misalignment math

### Definition of Done
- Player can earn tags through actions and jobs
- Jobs filter by required tags; misaligned jobs show penalties
- Job income flows into money each turn
- Growth jobs award tags after threshold

---

## Sprint 5: Event Card Content Pass (Weeks 10–11)

**Goal:** Full 40-card deck authored, balanced, and reskinnable.

### Tasks
- [ ] Author remaining event cards to reach 40 total
  - Stage A: 8 cards
  - Stage B: 12 cards
  - Stage C: 8 cards
  - Stage D: 12 cards
- [ ] Implement reskinning system — template events with stage-specific text variants
- [ ] Balance pass: playtest 5 full runs, adjust stat deltas
- [ ] Add rarity distribution tuning (common 60%, uncommon 30%, rare 10%)
- [ ] Add conditional options (e.g., "Ask mentor for help" only if Support ≥ 5)
- [ ] Add KC flavor text to events (bus routes, local employers, neighborhood names)
- [ ] Populate `data/kc-config.json` with wage bands, cost ranges, industry names

### Definition of Done
- 40 unique event cards in `data/events.json`
- At least 6 templates with stage-appropriate reskins
- Conditional options display only when requirements are met
- KC flavor is present but non-blocking (game works without it)

---

## Sprint 6: Endings + Timeline Recap (Weeks 12–13)

**Goal:** The game has meaningful conclusions and a reflection layer.

### Tasks
- [ ] Implement `ending_resolver.rs` — evaluate final state against 5 ending conditions
- [ ] Author ending text for all 5 outcomes in `data/endings.json`
- [ ] Build ending screen in frontend — display outcome, stats summary, narrative
- [ ] Implement timeline logic — extract top 8 impactful decisions from decision log
- [ ] Add `GET /api/timeline` and `GET /api/endings` endpoints
- [ ] Build timeline visualization (vertical list with turn #, choice, and consequence)
- [ ] Build reflection prompts UI — display 2–3 reflection questions
- [ ] Add "Play Again" button that calls `POST /api/new_game` (optionally with new seed)
- [ ] Add "Share Seed" button — copies seed to clipboard

### Definition of Done
- Every playthrough resolves to one of 5 endings
- Timeline shows the player's most impactful decisions
- Reflection prompts display after ending
- Player can restart or share their seed

---

## Sprint 7: Scenario Mode + Classroom Features (Weeks 14–15)

**Goal:** Teachers can use this in a classroom setting.

### Tasks
- [ ] Build scenario selector — preset starting conditions:
  - "No car" (no transport choices, bus-only)
  - "Helping family" (Support starts high, Money starts low, extra family events)
  - "Single parent household" (fewer time slots, unique decisions)
  - "Fresh start" (default — no modifiers)
- [ ] Implement `?seed=XXXX` query param support in Actix route
- [ ] Build simple "Teacher Setup" page — generate a seed, select scenario, get shareable link
- [ ] Add print-friendly timeline recap (CSS `@media print`)
- [ ] Add keyboard navigation for all interactive elements
- [ ] Accessibility audit: screen reader labels, focus management, color contrast

### Definition of Done
- 4 scenarios playable with distinct starting conditions
- Seed URL works — all students see same events with same seed
- Teacher can generate and distribute a link
- Passes WCAG 2.1 AA for keyboard and color contrast

---

## Sprint 8: Polish + Playtesting (Weeks 16–17)

**Goal:** The game feels good, is balanced, and bugs are squashed.

### Tasks
- [ ] Visual polish pass — consistent spacing, transitions, card animations
- [ ] Add stat change animations (CSS transitions: numbers tick up/down, color flash)
- [ ] Add event card draw animation (CSS slide in / flip)
- [ ] Sound effects (optional, toggle-able): card draw, stat change, stage transition
- [ ] Playtest with 5+ external testers — collect feedback
- [ ] Balance adjustments based on playtest data
- [ ] Bug bash: fix all P0/P1 issues from testing
- [ ] Performance check — ensure smooth play on low-end Chromebooks
- [ ] Final content review — typos, tone consistency, KC accuracy

### Definition of Done
- Animations are smooth and non-blocking
- No P0/P1 bugs
- Playtesters can complete a full run without confusion
- Page loads in < 3 seconds on target hardware

---

## Sprint 9: Launch Prep (Weeks 18–19)

**Goal:** Production-ready binary with documentation.

### Tasks
- [ ] Enable `rust-embed` for release builds — bake `static/` and `data/` into binary
- [ ] Test single-binary distribution: `cargo build --release` → copy binary → run
- [ ] Write teacher guide (PDF or web page):
  - How to set up a seed
  - Suggested discussion questions
  - Curriculum alignment notes
- [ ] Write student-facing "How to Play" overlay (in-game, dismissable)
- [ ] Add optional privacy-respecting analytics: completion rate, avg playtime, ending distribution
- [ ] Final accessibility check
- [ ] Create landing page with game description and "Play" button
- [ ] Test on target hardware: Chromebook, classroom projector, various browsers

### Definition of Done
- `cargo build --release` produces a single distributable binary
- Teacher guide is complete and shareable
- In-game tutorial is clear and non-intrusive
- Binary runs on Windows, macOS, and Linux

---

## Sprint 10: Post-Launch & Iteration (Week 20+)

**Goal:** Respond to real classroom use and plan v2.

### Tasks
- [ ] Collect teacher and student feedback
- [ ] Fix bugs reported from classroom use
- [ ] Analyze ending distribution — is one ending too common?
- [ ] Plan v2 features based on feedback:
  - More event cards
  - Additional scenarios
  - Multiplayer/comparison mode
  - Real KC wage/cost data integration
  - Spanish language support
  - Optional WASM frontend for pure-static deployment
- [ ] Document lessons learned

### Definition of Done
- All critical post-launch bugs fixed within 1 week
- Feedback synthesis document created
- v2 roadmap drafted

---

## Summary Timeline

| Sprint | Weeks | Milestone |
|--------|-------|-----------|
| 0 | 1 | Project scaffolded, `cargo run` serves placeholder |
| 1 | 2–3 | Engine runs turns with tests passing |
| 2 | 4–5 | **Playable Stage A in browser** |
| 3 | 6–7 | **Full 12-turn playthrough** |
| 4 | 8–9 | Credential + job system live |
| 5 | 10–11 | 40 event cards, KC flavor |
| 6 | 12–13 | Endings, timeline, reflection |
| 7 | 14–15 | Classroom features + scenarios |
| 8 | 16–17 | Polish + playtesting |
| 9 | 18–19 | **Production binary release** |
| 10 | 20+ | Post-launch iteration |

**Key milestone: Sprint 2 produces a playable demo.** Everything after that is content, systems, and polish layered on top of a working core.
