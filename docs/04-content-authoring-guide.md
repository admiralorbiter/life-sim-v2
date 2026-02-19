# Life Roguelite — Content Authoring Guide

This document defines how to write event cards, actions, decisions, jobs, and endings so that all content is consistent, balanced, and easy to maintain.

> **Note:** All JSON data files are deserialized into Rust structs via `serde`. Field names in JSON must use `camelCase` and match the schema exactly. The Rust structs use `#[serde(rename_all = "camelCase")]` to map from JSON camelCase to Rust snake_case.

---

## 1. Event Card Schema

Every event card is a JSON object in `data/events.json`.

```json
{
  "id": "evt_car_trouble_d",
  "title": "Car Trouble",
  "flavorText": "Your car won't start. You need it to get to work.",
  "stages": ["early-adult"],
  "rarity": "common",
  "options": [
    {
      "label": "Pay for repair",
      "description": "Shell out the cash and get back on the road.",
      "effects": [
        { "stat": "money", "delta": -200 }
      ]
    },
    {
      "label": "Ask your mentor for a ride",
      "description": "They'll help, but you'll owe a favor.",
      "effects": [
        { "stat": "support", "delta": -1 },
        { "stat": "stress", "delta": 5 }
      ],
      "requiresSupport": 3
    },
    {
      "label": "Take the bus this week",
      "description": "It works, but it eats into your schedule.",
      "effects": [
        { "stat": "timeSlots", "delta": -1 },
        { "stat": "stress", "delta": 5 }
      ]
    }
  ]
}
```

### Field Rules

| Field | Required | Notes |
|-------|----------|-------|
| `id` | Yes | Format: `evt_[short_name]_[stage_letter]`. Unique across all cards. |
| `title` | Yes | 2–5 words. |
| `flavorText` | Yes | 1–2 sentences. Second person ("You…"). Present tense. |
| `stages` | Yes | Array of 1+ stage IDs. |
| `rarity` | Yes | `common` (60%), `uncommon` (30%), `rare` (10%). |
| `options` | Yes | Array of 2–3 options. Never 1, never more than 3. |
| `options[].label` | Yes | Short imperative phrase (≤ 6 words). |
| `options[].description` | Yes | 1 sentence explaining what happens. |
| `options[].effects` | Yes | 1–3 stat effects. |
| `options[].delayedEffects` | No | Effects that trigger N turns later. |
| `options[].requiresSupport` | No | Minimum support to see this option. |

### Writing Guidelines

- **No moralizing.** Don't label options as "good" or "bad." Let the tradeoffs speak.
- **Every option has a cost.** Even the "best" option should cost something (time, money, stress, or support).
- **At least one cheap option.** Players in bad states need a survivable (if painful) path.
- **Use KC flavor when possible.** Reference bus routes, neighborhoods, local employers — but keep it light. The game should still make sense without KC knowledge.
- **Reskin-friendly.** If a card could work across stages with different flavor text, write it as a template (see Reskinning below).

---

## 2. Reskinnable Event Templates

Some events are universal concepts that recur at different life stages. Author these as templates with stage-specific variants.

### Template Format

```json
{
  "templateId": "tmpl_transport_issue",
  "title": "Getting Around",
  "variants": {
    "middle-school": {
      "id": "evt_bike_broken_a",
      "flavorText": "Your bike tire is flat and you need to get to school.",
      "options": [
        { "label": "Buy a new tube", "effects": [{ "stat": "money", "delta": -15 }] },
        { "label": "Walk this week", "effects": [{ "stat": "timeSlots", "delta": -1 }, { "stat": "stress", "delta": 5 }] },
        { "label": "Ask a friend's parent for rides", "effects": [{ "stat": "support", "delta": -1 }] }
      ]
    },
    "high-school": {
      "id": "evt_bus_cancelled_b",
      "flavorText": "Your bus route got cancelled. School is 4 miles away.",
      "options": [
        { "label": "Buy a monthly bus pass", "effects": [{ "stat": "money", "delta": -50 }] },
        { "label": "Carpool with a classmate", "effects": [{ "stat": "support", "delta": -1 }, { "stat": "stress", "delta": 3 }] },
        { "label": "Bike (it's far)", "effects": [{ "stat": "timeSlots", "delta": -1 }, { "stat": "stress", "delta": 8 }] }
      ]
    },
    "early-adult": {
      "id": "evt_car_trouble_d",
      "flavorText": "Your car won't start. You need it to get to work.",
      "options": ["(see full example above)"]
    }
  }
}
```

### Core Templates to Author

| Template | Concept | Stages |
|----------|---------|--------|
| Transportation issue | Getting to school/work breaks down | A, B, C, D |
| Unexpected expense | Something costs money you didn't plan for | A, B, C, D |
| Social conflict | Friction with friend/peer/roommate | A, B, D |
| Opportunity knocks | A chance to gain skills/money/connections | B, C, D |
| Burnout warning | Stress is catching up to you | B, D |
| Family needs | Family asks for your time or money | A, C, D |

---

## 3. Action Definitions

Actions are what players pick in Phase 1 (Plan). Defined in `data/actions.json`.

```json
{
  "id": "act_study",
  "label": "Study",
  "description": "Hit the books. Reduces stress slightly, builds toward credentials.",
  "stages": ["middle-school", "high-school", "post-high", "early-adult"],
  "effects": [
    { "stat": "stress", "delta": -3 },
    { "stat": "credentials", "delta": 0, "progressToward": "next-available" }
  ],
  "timeCost": 1
}
```

### Action Pool by Stage

| Stage | Available Actions |
|-------|-------------------|
| **A: Middle School** | Study, Friends, Family, Rest, Clubs |
| **B: High School** | Study, Work, Friends, Family, Rest, Clubs, Skill-building |
| **C: Post-High** | Study, Work, Family, Rest, Skill-building |
| **D: Early Adult** | Work, Friends, Family, Rest, Skill-building, Side Hustle |

---

## 4. Decision Definitions

Decisions are the Phase 2 (Commit) choices. Defined in `data/decisions.json`.

```json
{
  "id": "dec_course_track_b",
  "stage": "high-school",
  "turn": 1,
  "prompt": "Which course track do you want to follow this year?",
  "options": [
    {
      "label": "Honors / AP",
      "effects": [{ "stat": "stress", "delta": 10 }],
      "grantsTag": "AP Credit",
      "description": "Harder coursework, but colleges notice."
    },
    {
      "label": "CTE / Vocational",
      "effects": [{ "stat": "stress", "delta": 3 }],
      "grantsTag": "CTE Certificate",
      "description": "Hands-on skills that lead to jobs."
    },
    {
      "label": "Standard",
      "effects": [{ "stat": "stress", "delta": 0 }],
      "description": "Keep it manageable."
    }
  ]
}
```

---

## 5. Job Definitions

Defined in `data/jobs.json`.

```json
{
  "id": "job_helpdesk",
  "title": "Helpdesk Technician",
  "requiredTags": ["IT Fundamentals"],
  "recommendedTags": ["Customer Service"],
  "payPerTurn": 80,
  "stressPerTurn": 3,
  "growthRate": 3,
  "growthTag": "IT Support Specialist",
  "stages": ["early-adult"],
  "description": "Answer tickets, reset passwords, learn on the job."
}
```

### Balance Guidelines for Jobs

| Pay Tier | Pay/Turn | Stress/Turn | Tags Required |
|----------|----------|-------------|---------------|
| Low | 30–50 | 5–8 | 0 |
| Medium | 60–90 | 3–6 | 1 |
| High | 100–140 | 2–5 | 2+ |

Low-skill jobs pay less and stress more. This is the core "credential alignment" lesson.

---

## 6. Ending Definitions

Defined in `data/endings.json`.

```json
{
  "id": "ending_stable",
  "title": "Stable + Credentialed",
  "conditions": {
    "money": { "min": 200 },
    "credentials": { "minCount": 3 },
    "stress": { "max": 50 }
  },
  "narrative": "You built a solid foundation. You have skills employers want, money in the bank, and room to breathe. It wasn't easy, but your planning paid off.",
  "reflection": "What choices early on set you up for this outcome?"
}
```

### Ending Priority Order

If multiple endings match, the resolver picks the first match in this order:

1. Stable + Credentialed
2. Stressed but Employed
3. Supported but Broke
4. Skilled but Isolated
5. Off-track but Recovering (default fallback)

---

## 7. Balance Cheat Sheet

Use these ranges when authoring content to keep the game fair:

| Stat | Starting Value (Stage A) | Typical Swing per Event | Danger Threshold |
|------|--------------------------|------------------------|------------------|
| Money | 100 | ±15–200 | < 50 |
| Stress | 20 | ±3–15 | > 75 |
| Support | 5 | ±1–2 | < 3 |
| Time Slots | 3 | ±1 (temporary) | N/A |

### Golden Rules
- No single event should swing Money by more than **200** or Stress by more than **20**.
- Every negative event must have at least one option costing ≤ 50 Money.
- Rare events can be more impactful but should never be game-ending alone.
- Delayed effects should trigger within **2–3 turns** — longer and players forget.
