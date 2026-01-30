# Contributing

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- A C linker (required by some Rust crates):
  - **Ubuntu/Debian**: `sudo apt install build-essential`
  - **Fedora/RHEL**: `sudo dnf install gcc`
  - **Arch**: `sudo pacman -S base-devel`
  - **macOS**: `xcode-select --install`

### Build Commands

```bash
# Development build (fast compilation, slower runtime)
cargo build

# Production build (optimized with LTO)
cargo build --release
```

### Running the Application

```bash
# Quick dev run (unoptimized)
cargo run -- <command>

# Production run (optimized)
cargo run --release -- <command>

# Examples:
cargo run -- view "Molten Whip"
cargo run -- rank --limit 20
cargo run --release -- optimize --classes Dragonknight -v
```

### Testing

```bash
# Run all unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_count_combinations
```

### Benchmarks

```bash
# Run performance benchmarks (requires release profile)
cargo bench
```

### Code Quality

```bash
# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Useful Commands

```bash
# Show help for all CLI commands
cargo run -- --help

# Show help for a specific command
cargo run -- optimize --help

# View skill details
cargo run -- view "Fiery Breath"

# Rank top 10 skills by damage
cargo run -- rank --limit 10

# Rank skills excluding ultimates
cargo run -- rank --exclude-ultimates

# Find optimal build for a specific class
cargo run --release -- optimize --classes Dragonknight

# Find optimal build with specific weapons
cargo run --release -- optimize --weapons Bow,DualWield
```

---

## Adding Skills

### Source Data

Skill data is extracted from [ESO Hub Skills](https://eso-hub.com/en/skills/). Each class has skill lines that can be found at URLs like:

- `https://eso-hub.com/en/skills/nightblade/shadow`
- `https://eso-hub.com/en/skills/nightblade/assassination`
- `https://eso-hub.com/en/skills/nightblade/siphoning`

Copy the skill descriptions from the relevant ESO Hub page and paste them into `RAW.md` in the repository root.

### Using the /add-skills Command

Once you have the raw skill data in `RAW.md`, use Claude Code's `/add-skills` command to generate the skill definitions:

```bash
/add-skills <class>/<skill-line>
```

Examples:

```bash
/add-skills nightblade/shadow
/add-skills nightblade/assassination
/add-skills nightblade/siphoning
```

The command will:

1. Read the raw skill data from `RAW.md`
2. Read the target skill file to understand the existing structure
3. Cross-reference to identify missing skills
4. Add the missing skills following the project conventions
5. Run `cargo check` to ensure no compilation errors
