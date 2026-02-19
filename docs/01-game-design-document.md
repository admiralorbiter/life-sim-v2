# Life Roguelite — Game Design Document

## 1. Overview

| Field | Detail |
|-------|--------|
| **Title** | Life Roguelite (working title) |
| **Genre** | Life Roguelite / Choice-Driven Sim |
| **Platform** | Desktop app (Rust binary serving local web UI; responsive) |
| **Target Audience** | Middle & high school students; classroom use |
| **Session Length** | 20–30 minutes per playthrough |
| **Core Pitch** | You start as a middle schooler and progress through 4 life stages. Each stage is a few turns. You make choices, draw Life Event cards, and keep your life stable while pursuing goals. The game teaches tradeoffs, recovery, and planning — not "pick the perfect path." |

---

## 2. Design Pillars

1. **Tradeoffs over optimization** — You can't max everything. Every choice costs something.
2. **Consequences compound** — Early habits echo into later stages. Delayed feedback is a core mechanic.
3. **Recovery is always possible** — No single bad draw ends the game. Resilience is the skill being taught.
4. **Show, don't preach** — Lessons emerge from play, not from text boxes.
5. **KC grounded** — Flavor, industries, and costs reflect Kansas City without requiring hard data.

---

## 3. Core Loop

Each **turn** (representing a week or semester depending on stage) follows four phases:

### Phase 1: Plan (Allocate Time)
- Player selects **2–3 actions** from a pool (pool varies by stage):
  - Study / Work / Family / Friends / Rest / Clubs / Skill-building
- Actions consume the turn's **Time** budget.

### Phase 2: Commit (Make a Decision)
- One key decision per turn tied to the current stage:
  - Course level, job choice, extracurricular, saving vs. spending, transportation, etc.

### Phase 3: Event (Draw a Life Card)
- A random Life Event card is drawn from the stage-appropriate deck.
- Each card presents **2–3 response options** with clear tradeoffs (money/time/stress/support).

### Phase 4: Feedback
- **Immediate:** Cash changes, stress changes shown on-screen.
- **Delayed:** GPA, skills, and opportunity unlocks shift in future turns.

---

## 4. Player Stats (Resources)

Five core resources. The player can never maximize all simultaneously.

| Stat | Description | Range | Danger Zone |
|------|-------------|-------|-------------|
| **Money** | Cash on hand. Later stages add monthly bills. | 0–999 | < 50 |
| **Time** | Actions available per turn (2–3 base). | 2–4 | N/A (hard cap) |
| **Stress** | Accumulated pressure. High stress degrades outcomes. | 0–100 | > 75 |
| **Skills / Credentials** | Tags earned (e.g., "IT Fundamentals", "CPR"). Unlock jobs/programs. | Tag list | 0 tags by Stage C |
| **Support** | Family / friends / mentor network strength. Buffers bad events. | 0–10 | < 3 |

### Stat Interactions
- **Stress > 75:** Random chance of "missed day" event; all action outcomes slightly worse.
- **Support > 7:** Some negative events gain a free mitigation option.
- **Money < 0:** Triggers "debt" card next turn; locks out some choices.

---

## 5. Life Stages

### Stage A: Middle School (3–4 turns)
**Focus:** Identity + habits

| Choices | Events |
|---------|--------|
| Join a club (tech, arts, sports) | Family schedule changes |
| Attendance / effort habits | New teacher notices you (+Support) |
| Friend group selection | Phone breaks (money tradeoff) |
| Mentor opportunities | |

### Stage B: High School (5–6 turns)
**Focus:** Pathways + compounding choices

| Choices | Events |
|---------|--------|
| Course track (honors / AP / CTE / dual credit) | Transportation problem |
| Extracurricular depth vs. breadth | "You bombed a test" (tutoring costs) |
| Summer choice (work / camp / volunteering / skill course) | Part-time job offer vs. unpaid internship |

### Stage C: Post-High Decision (2–3 turns)
**Focus:** Decision under uncertainty

| Paths Available | Events |
|-----------------|--------|
| Community college | FAFSA surprise / aid changes |
| 4-year university | Housing shift |
| Workforce (direct) | Family needs help |
| Apprenticeship / training program | |
| Gap year (structured) | |

### Stage D: Early Adult (5–6 turns)
**Focus:** Budgeting + resilience

| New Systems | Events |
|-------------|--------|
| Monthly budget (rent / phone / food / transport) | Car repair / medical bill |
| Emergency fund mechanic | Roommate moves out |
| Credential → job alignment matters | "Promotion if you get X cert" |
| | Burnout (if stress unmanaged) |

---

## 6. Credential / Job System

### Tags
Courses, clubs, and training grant **tags**:
- Examples: `IT Fundamentals`, `Customer Service`, `CPR`, `Dual Credit`, `Portfolio`, `Welding Cert`

### Job Requirements
Each job lists required and recommended tags:

```
Helpdesk Technician
  Required: [IT Fundamentals]
  Recommended: [Customer Service]
  Pay: Medium
  Stress: Low
  Growth: +1 tag slot per 3 turns

Warehouse Associate
  Required: []
  Recommended: [Forklift Cert]
  Pay: Low-Medium
  Stress: Medium
  Growth: None
```

### Misalignment Penalty
Taking a job without recommended tags → lower pay modifier, +Stress per turn, fewer growth events.

---

## 7. Life Event Cards — Design Spec

Each card contains:
- **Title** and **flavor text**
- **2–3 response options**, each with explicit stat changes
- **Stage tag** (which stages this card can appear in)
- **Rarity** (common / uncommon / rare)

### Reskinning System
Core event templates scale across stages:

| Template | Stage A | Stage B | Stage C–D |
|----------|---------|---------|-----------|
| Transportation issue | Bike breaks | Bus route cancelled | Car needs repair |
| Unexpected expense | Phone breaks | Test prep book | Medical bill |
| Social conflict | Friend drama | Group project tension | Roommate conflict |
| Opportunity | Club leadership | Internship offer | Promotion track |

### Example Card: Car Trouble (Stage D)

> **Car Trouble**
> Your car won't start. You need it to get to work.
>
> - **Pay for repair now** → Money -200, no other effect
> - **Borrow a ride from mentor** → Support -1, Stress +5 next turn (owe a favor)
> - **Take the bus this week** → Time -1 this turn, Stress +5, Money unchanged

---

## 8. Endings

Five outcome profiles (not ranked good/bad):

| Ending | Condition |
|--------|-----------|
| **Stable + Credentialed** | Money > threshold, 3+ relevant tags, Stress < 50 |
| **Stressed but Employed** | Has job, Stress > 70, Money okay |
| **Supported but Broke** | Support > 7, Money < threshold |
| **Skilled but Isolated** | 4+ tags, Support < 3 |
| **Off-track but Recovering** | Failed a stage goal but trending upward in final turns |

Each ending shows a **timeline recap** and **reflection prompts**.

---

## 9. Classroom Features

- **Timeline recap:** "Here are the 8 decisions that mattered most."
- **Reflection prompts** (optional button after ending):
  - "What would you do differently with the same random events?"
  - "Which stat was hardest to manage and why?"
- **Scenario mode:** Start with preset constraints (no car, helping family, single parent, etc.)
- **Seed sharing:** Teacher can share a random seed so all students face the same events.

---

## 10. KC Flavor (Config-Driven)

All location-specific data lives in a single `data/kc-config.json`:
- Transportation options and costs (KCATA bus pass, car insurance ranges)
- Local industries: healthcare, logistics, construction trades, IT
- Community college vs. training program names
- Wage ranges as Low / Medium / High bands

This keeps the game balanced and easy to re-skin for other cities later.
