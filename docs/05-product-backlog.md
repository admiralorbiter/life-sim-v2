# Life Roguelite — Product Backlog

Stories are grouped by epic and roughly ordered by sprint. Priority: **P0** = must-have for MVP, **P1** = important, **P2** = nice-to-have.

---

## Epic 1: Core Engine

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E1-01 | As a developer, I can initialize a `GameState` struct with default stats so that the engine has a starting point. | P0 | 0 | S |
| E1-02 | As a developer, I can run a seeded RNG (`ChaCha8Rng`) so that classroom sessions are reproducible. | P0 | 0 | S |
| E1-03 | As a developer, I can apply stat changes with clamping so that stats never go out of bounds. | P0 | 1 | M |
| E1-04 | As a developer, I can draw an event card filtered by stage and rarity so that events are contextually appropriate. | P0 | 1 | M |
| E1-05 | As a developer, I can execute a full turn (plan → commit → event → feedback) via `turn_runner.rs` so that game progression works. | P0 | 1 | L |
| E1-06 | As a developer, I can transition between stages when turn count is reached so that the game progresses. | P0 | 3 | M |
| E1-07 | As a developer, I can apply delayed effects on the correct future turn. | P0 | 3 | M |
| E1-08 | As a developer, I can trigger stress threshold effects (missed day) when stress > 75. | P0 | 1 | S |
| E1-09 | As a developer, I can apply monthly bills in Stage D each turn. | P0 | 3 | S |
| E1-10 | As a developer, I can resolve the game into one of 5 endings based on final state via `ending_resolver.rs`. | P0 | 6 | M |

---

## Epic 2: REST API & Web UI

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E2-01 | As a player, I can see my current stats (money, stress, support, time, credentials) at all times. | P0 | 2 | M |
| E2-02 | As a player, I can select 2–3 actions in the Plan phase via the browser UI. | P0 | 2 | M |
| E2-03 | As a player, I can choose a decision in the Commit phase. | P0 | 2 | M |
| E2-04 | As a player, I can read an event card and pick a response option. | P0 | 2 | M |
| E2-05 | As a player, I can see immediate stat changes after each phase. | P0 | 2 | S |
| E2-06 | As a player, I can see a stage transition summary between stages. | P0 | 3 | M |
| E2-07 | As a player, I can see my ending screen with narrative text and final stats. | P0 | 6 | M |
| E2-08 | As a player, I can see a timeline recap of my most impactful decisions. | P1 | 6 | L |
| E2-09 | As a player, I can see reflection prompts after the ending. | P1 | 6 | S |
| E2-10 | As a player, I can play the game on a Chromebook (1366×768). | P0 | 2 | M |
| E2-11 | As a player, I can navigate the entire game with keyboard only. | P1 | 7 | M |
| E2-12 | As a player, I see smooth CSS animations for stat changes and card draws. | P2 | 8 | L |
| E2-13 | As a developer, I can start the game server with `cargo run` and access it at `localhost:8080`. | P0 | 0 | M |
| E2-14 | As a developer, I have REST API endpoints for all game phases (`new_game`, `submit_actions`, `submit_decision`, `draw_event`, `submit_event_response`, `advance_turn`). | P0 | 2 | L |

---

## Epic 3: Credential & Job System

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E3-01 | As a player, I earn credential tags from actions, decisions, and jobs. | P0 | 4 | M |
| E3-02 | As a player, I can see which jobs I qualify for based on my tags. | P0 | 4 | M |
| E3-03 | As a player, I experience lower pay and higher stress if I take a job I'm not qualified for. | P0 | 4 | M |
| E3-04 | As a player, I earn a new tag after working a growth job for N turns. | P1 | 4 | M |
| E3-05 | As a player, I can see a "job board" showing available jobs and their requirements. | P1 | 4 | S |

---

## Epic 4: Content

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E4-01 | Game has 5 event cards for Stage A (initial testing). | P0 | 1 | M |
| E4-02 | Game has full action/decision sets for all 4 stages. | P0 | 3 | L |
| E4-03 | ✅ Game has 40 event cards across all stages with balanced stat effects. | P0 | 5 | XL |
| E4-04 | ✅ Event cards use reskinnable templates (6 template concepts as flat variants). | P1 | 5 | M |
| E4-05 | ✅ Events include conditional options gated by support level. | P1 | 5 | S |
| E4-06 | ✅ Content includes KC-specific flavor (employers, transit, neighborhoods). | P1 | 5 | M |
| E4-07 | All 5 endings have narrative text and reflection questions. | P0 | 6 | M |
| E4-08 | 8–10 jobs authored with tag requirements and balance targets. | P0 | 4 | M |

---

## Epic 5: Classroom Features

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E5-01 | As a teacher, I can share a seed URL (`?seed=XXXX`) so all students face the same events. | P1 | 7 | M |
| E5-02 | As a teacher, I can select a scenario with preset starting conditions. | P1 | 7 | L |
| E5-03 | As a teacher, I can generate a shareable link from a setup page. | P1 | 7 | M |
| E5-04 | As a student, I can print my timeline recap. | P2 | 7 | S |
| E5-05 | As a teacher, I have a written guide with discussion questions and curriculum notes. | P1 | 9 | L |

---

## Epic 6: Polish & Launch

| ID | Story | Priority | Sprint | Estimate |
|----|-------|----------|--------|----------|
| E6-01 | Game has consistent visual design with clear typography and spacing. | P1 | 8 | L |
| E6-02 | Stat changes animate smoothly (CSS transitions, color flashes). | P2 | 8 | M |
| E6-03 | Event cards have a draw/flip CSS animation. | P2 | 8 | M |
| E6-04 | Game has optional, toggle-able sound effects. | P2 | 8 | M |
| E6-05 | Game loads in under 3 seconds on target hardware. | P1 | 8 | S |
| E6-06 | Game passes WCAG 2.1 AA for color contrast. | P1 | 7 | M |
| E6-07 | `cargo build --release` produces a single distributable binary with embedded assets. | P0 | 9 | M |
| E6-08 | In-game "How to Play" overlay for first-time players. | P1 | 9 | M |
| E6-09 | Privacy-respecting analytics (completion rate, ending distribution). | P2 | 9 | M |

---

## Sizing Key

| Size | Meaning | Rough Hours |
|------|---------|-------------|
| S | Small | 1–3 hours |
| M | Medium | 4–8 hours |
| L | Large | 8–16 hours |
| XL | Extra Large | 16–32 hours |
