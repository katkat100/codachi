# Codachi

A terminal Tamagotchi pet that reacts to your code! Watch your little companion thrive when you write clean code, or get grumpy when bugs pile up.

```
              █████
            ██     ██
          ██         ██
        ██             ██
        ██             ██
      ██                 ██
      ██                 ██
    ██                     ██
    ██                     ██
    ██                     ██
    ██                     ██
      ██                 ██
      ██                 ██
        ██             ██
          ███       ███
             ███████
```

## Installation

### From source
```bash
git clone https://github.com/katkat100/codachi.git
cd codachi
cargo build --release
```

### Run it
```bash
# Watch the current directory
cargo run

# Watch a specific project
cargo run -- --watch /path/to/your/project
```

## Controls

| Key | Action |
|-----|--------|
| `F` | Feed your pet (costs 1 point) |
| `C` | Clean your pet |
| `A` | View achievements |
| `Q` | Quit |

## How It Works

Codachi watches your project files and reacts to your coding:

- **Clean saves** (no errors/warnings) → Pet gains health and XP
- **Errors in code** → Pet loses health and cleanliness
- **Warnings** → Minor damage to health and cleanliness
- **Fixing errors** → Pet regains health
- **Git commits** → Earn points to feed your pet

## Stats

### Bars

| Stat | What it does | How it changes |
|------|--------------|----------------|
| **Health** | Overall wellbeing | +1 per clean save, +3 per error fixed, -5 per error, -2 per warning |
| **Hunger** | Needs regular feeding | -1 every 15 minutes, +20 when fed (costs 1 point) |
| **Cleanliness** | Affected by code quality | +25 when cleaned, -3 per error, -1 per warning |

### Points & XP

- **Points**: Earned from git commits, spent on feeding
- **XP**: Earned from clean saves (+5), feeding (+3), cleaning (+3), and active coding time

## Levels

Your pet evolves as it gains XP:

| Level | Name | XP Required |
|-------|------|-------------|
| 1 | Egg | 0 |
| 2 | Buddy | 500 |
| 3 | Elder | 1500 |

**Note**: Your pet must have at least 30 health to level up!

## Moods

Your pet's mood changes based on its stats (checked in priority order):

| Mood | Trigger |
|------|---------|
| **Sick** | Health below 30, OR 2+ stats below 20 |
| **Grumpy** | Hunger below 20, OR Cleanliness below 20 |
| **Sleepy** | No interaction for 60+ minutes |
| **Excited** | All stats between 80-100, or after achievements |
| **Happy** | Default state when well cared for |

## Custom Sprites

You can customize your pet's appearance! Place ASCII art files in your project's `ascii/` or `.codachi/sprites/` folder:

```
ascii/
  egg-idle.txt
  egg-idle-2.txt      # Animation frames
  egg-sick.txt
  egg-grumpy.txt      # or egg-angry.txt
  egg-sleepy.txt      # or egg-sleep.txt
  egg-excited.txt     # or egg-excite.txt
  buddy-idle.txt
  buddy-sick.txt
  ...
```

Sprites hot-reload when you save changes - no restart needed!

## Language Support

Codachi automatically detects your project type and configures linting:

| Language | Detection | Lint Command |
|----------|-----------|--------------|
| **Rust** | `Cargo.toml` | `cargo check --message-format=json` |
| **TypeScript** | `tsconfig.json` | `npx tsc --noEmit` |
| **JavaScript** | `package.json` | `npx eslint . --format json` |
| **Python** | `requirements.txt`, `pyproject.toml` | `python -m py_compile` |
| **Go** | `go.mod` | `go build ./...` |
| **Ruby** | `Gemfile` | `ruby -c` |
| **Java** | `pom.xml`, `build.gradle` | `javac -Xlint:all` |
| **C#** | `*.csproj`, `*.sln` | `dotnet build` |
| **C++** | `CMakeLists.txt`, `Makefile` | `make` |

## Configuration

Codachi works out of the box with auto-detection, but you can customize by creating `.codachi/config.toml`:

```toml
# Lint command to run on save
lint_cmd = "cargo check --message-format=json"

# Parser type: "cargo", "eslint", or "regex"
lint_parser = "cargo"

# File patterns to watch
watch_patterns = ["**/*.rs", "**/*.toml"]

# For regex parser, specify patterns
error_pattern = "error:"
warning_pattern = "warning:"
```

## Test Mode

For testing sprites and mechanics:

```bash
cargo run -- --test-mode
```

Test mode commands:

| Key | Action |
|-----|--------|
| `1` | Cycle level (Egg → Buddy → Elder → Egg) |
| `2` | Add 100 points |
| `3` | Damage health (-30) |
| `4` | Heal (+30) |
| `5` | Reduce cleanliness (-40) |
| `6` | Reduce hunger (-40) |
| `7` | Add 100 XP |
| `0` | Reset to defaults |

## License

MIT
