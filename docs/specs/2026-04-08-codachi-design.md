# Codachi - Design Spec

A retro pixel art Tamagotchi-style pet that lives in your terminal. Run it in Zed's built-in terminal panel and it reacts to your coding -- errors make it sick, commits earn points, and consistent care levels it up.

**Personality:** Cute and wholesome with occasional sassy remarks. Retro pixel art aesthetic rendered via Unicode block/braille characters.

**Tech Stack:** Rust, ratatui (TUI framework), notify (filesystem watcher), serde (serialization)

---

## Architecture

### Application Type
- Standalone Rust terminal application using `ratatui` for persistent TUI rendering
- Runs inside Zed's built-in terminal panel (or any terminal emulator)
- Single binary, installed via `cargo install` or direct download
- Launched with `codachi` from the project root (or `codachi --watch /path/to/project`)

### Persistent State
Stored as `.codachi/state.json` in the workspace root.

```json
{
  "pet": {
    "name": "Codachi",
    "level": 1,
    "xp": 0,
    "health": 100,
    "hunger": 100,
    "cleanliness": 100,
    "mood": "happy",
    "alive_since": "2026-04-08T12:00:00Z"
  },
  "economy": {
    "points": 0,
    "total_commits": 0,
    "last_known_commit_count": 0
  },
  "achievements": [],
  "session": {
    "errors_since_last_save": 0,
    "warnings_since_last_save": 0,
    "clean_saves_streak": 0,
    "last_interaction": "2026-04-08T12:00:00Z"
  }
}
```

### Configuration
Stored as `.codachi/config.toml` in the workspace root.

```toml
# Command to run for error/warning detection after file saves
lint_cmd = "cargo check --message-format=json"

# File patterns to watch for saves (glob syntax)
watch_patterns = ["**/*.rs", "**/*.toml"]

# How to parse lint output: "cargo", "eslint", "gcc", "regex"
lint_parser = "cargo"

# Custom regex parser (if lint_parser = "regex")
# error_pattern = "^error"
# warning_pattern = "^warning"
```

### Event Sources
- **File save detection:** `notify` crate watches the workspace for file write events matching `watch_patterns`. Debounced (500ms) to avoid rapid re-triggers.
- **Error/warning detection:** After a save is detected, runs the configured `lint_cmd`, parses stdout/stderr for error and warning counts using the configured parser.
- **Commit detection:** Periodic polling (every 30 seconds) of `git rev-list --count HEAD`. Delta from `last_known_commit_count` = new commits = new points.
- **Time tracking:** App tracks its own uptime as active time. Hunger decays based on this.

### Rendering
- `ratatui` TUI framework with `crossterm` backend
- Pixel art sprites rendered using Unicode half-block characters (▀▄█) for a chunky retro pixel look
- Animation loop running at ~4 FPS (250ms per frame) for sprite animations
- Input handling via `crossterm` events for keyboard shortcuts (f = feed, c = clean, a = achievements, q = quit)

---

## Pet Evolution

Three levels, each with a distinct visual form.

| Level | Name | XP Required | Visual Description |
|-------|------|-------------|-------------------|
| 1 | Sprout | 0 | Small blob with simple face. Limited animations (bounce, blink). |
| 2 | Buddy | 500 XP | Bigger blob with arms. More expressions and animations (wave, stretch, look around). |
| 3 | Elder | 1500 XP | Full creature with legs, crown/horns. Special idle animations (juggle, meditate, flex). |

**Evolution gate:** Pet cannot level up while health is below 30. XP still accumulates, but the transformation is held until health recovers.

---

## Core Stats

### Health (0-100)
Affected by code quality on save.

| Event | Effect |
|-------|--------|
| Warning on save | -2 health, pet sneezes |
| Error on save | -5 health, pet gets visibly sick |
| Error fixed (error count decreased on next save) | +3 health per error fixed |
| Clean save (0 errors, 0 warnings) | +1 health |
| Hunger at 0 | -1 health per 15 min (starvation damage) |

At 0 health: pet is bedridden, no idle animations, only guilt-trip remarks.

### Hunger (0-100)
Time-based decay.

| Event | Effect |
|-------|--------|
| Active time (app running) | -1 per 15 minutes |
| Feed (costs 1 point) | +20 hunger |

At 0 hunger: mood locks to grumpy, health starts decaying.

### Cleanliness (0-100)
Event-based decay tied to code quality.

| Event | Effect |
|-------|--------|
| Save with errors | -3 cleanliness |
| Save with warnings | -1 cleanliness |
| Clean (free, no point cost) | +25 cleanliness |

Cleaning cooldown: once every 5 minutes.
At 0 cleanliness: pet visibly grubby, mood locks to grumpy.

### Stat Interactions
- Any two stats below 20: mood locks to "sick"
- All three stats above 80: mood can reach "excited"
- Health below 30: blocks level-up

---

## Points Economy

| Source | Points |
|--------|--------|
| 1 git commit | +1 point |

| Action | Cost |
|--------|------|
| Feed pet | 1 point |
| Clean pet | Free |

---

## XP Sources

| Action | XP |
|--------|-----|
| Clean save (0 errors) | +5 |
| Commit | +10 |
| Feeding | +3 |
| Cleaning | +3 |
| Active time | +1 per 10 minutes |

---

## Mood System

### Mood States

| Mood | Trigger | Priority |
|------|---------|----------|
| Sick | Health < 30 OR any two stats < 20 | 1 (highest) |
| Grumpy | Hunger or cleanliness < 20 | 2 |
| Sleepy | 60+ min continuous app time without interaction | 3 |
| Excited | All stats > 80, or just leveled up, or achievement unlocked | 4 |
| Happy | All stats > 60 (default state) | 5 (lowest) |

Highest-priority matching mood wins.

### Sassy Remarks
Speech bubble rendered next to the pet sprite. One remark at a time, visible for ~8 seconds then clears. Randomly selected from pools per event type.

**Remark Pools:**

- **Error save:** "Yikes.", "That's... a choice.", "My immune system can't keep up with your syntax."
- **Warning save:** "*achoo!*", "Something smells off.", "I'll survive... probably."
- **Clean save:** "Look at you go!", "Clean code? I could get used to this.", "Finally, peace."
- **Fix (errors resolved):** "I can breathe again!", "The cure was in you all along.", "My hero."
- **Feeding:** "Nom nom nom!", "You DO care!", "Best. Snack. Ever."
- **Cleaning:** "Ahh, fresh!", "I was starting to stick to things.", "Squeaky clean blob reporting for duty."
- **Starving (hunger = 0):** "Hello? Feed me?", "I'm wasting away over here.", "You have mass commits but zero snacks?"
- **Neglect (no interaction 30+ min):** "Still here btw.", "Remember me?", "I'll just... sit here then."
- **Level up:** "EVOLUTION TIME!", "I feel... POWERFUL.", "New form, who dis?"

---

## Idle Behaviors & Animations

### Animation Approach
Pixel art sprites rendered using Unicode half-block characters (▀▄█). Each animation is a sequence of 2-4 frames cycling at ~4 FPS. Classic Tamagotchi style.

### Idle Animations by Mood

| Mood | Animations |
|------|-----------|
| Happy | Gentle bounce, looking around, blinking |
| Excited | Quick bounce, spinning, sparkle particles |
| Grumpy | Foot tapping, turning away, side-eye |
| Sleepy | Head nodding, Z's floating up, slow blink |
| Sick | Shivering, lying down, sweat drops |

### Idle Animations by Level
- **Level 1 (Sprout):** Bounce and blink only
- **Level 2 (Buddy):** Adds arm waves, looking left/right, stretching
- **Level 3 (Elder):** Adds juggling, flexing, sitting cross-legged meditation, crown polish

### Event-Triggered Animations
These override idle and play once before returning to the idle loop.

| Event | Animation | Duration |
|-------|-----------|----------|
| Error save | Flinch + sneeze/sickness | 1-2 sec |
| Clean save | Fist pump or nod | 1 sec |
| Feeding | Eating with crumbs | 2 sec |
| Cleaning | Shake-off with sparkles | 2 sec |
| Level up | Glow + transformation | 3-4 sec |
| Achievement | Trophy pops above head | 2 sec |

---

## Achievements

### Code Quality
| Achievement | Condition |
|------------|-----------|
| First Clean Save | Save with 0 errors/warnings |
| On a Roll | 5 clean saves in a row |
| Spotless | 25 clean saves in a row |
| Bug Squasher | Fix 10 errors total |
| Exterminator | Fix 50 errors total |

### Commitment
| Achievement | Condition |
|------------|-----------|
| First Commit | First commit with Codachi alive |
| Streak of 5 | 5 commits in one day |
| Centurion | 100 lifetime commits |

### Caretaking
| Achievement | Condition |
|------------|-----------|
| First Meal | Feed Codachi for the first time |
| Spa Day | Clean Codachi for the first time |
| Attentive Parent | All stats above 80 for 1 hour |
| Neglectful | Let any stat hit 0 |

### Milestones
| Achievement | Condition |
|------------|-----------|
| It's Alive! | First launch (hatch) |
| Growing Up | Reach Level 2 |
| Final Form | Reach Level 3 |
| Veteran | Codachi alive for 7 days |
| Old Friends | Codachi alive for 30 days |

### Display
- Pet speaks a remark when an achievement unlocks
- Recent achievements shown at bottom of TUI
- Full list accessible via 'a' key

---

## TUI Layout

```
┌─────────────────────────────────┐
│          C O D A C H I          │
│   ★ Level 1 - Sprout  😊 Happy │
├─────────────────────────────────┤
│                                 │
│         ██  ██                  │
│        ████████                 │
│        █ ▀▀ █                   │
│        ████████    "Look at     │
│         ██████      you go!"    │
│                                 │
├─────────────────────────────────┤
│ Health:    ████████░░ 85%       │
│ Hunger:    ██████░░░░ 60%       │
│ Cleanliness:███████░░░ 70%      │
├─────────────────────────────────┤
│ Points: 12  │  XP: ████░░ 340  │
├─────────────────────────────────┤
│ [F]eed (1pt)  [C]lean  [A]chv  │
├─────────────────────────────────┤
│ 🏆 First Clean Save!           │
│ 🔥 5 Commit Streak             │
└─────────────────────────────────┘
```

### Keyboard Controls
- `f` -- Feed pet (costs 1 point)
- `c` -- Clean pet (free, 5 min cooldown)
- `a` -- Toggle achievements list
- `q` -- Quit (state auto-saves)
