# ESO Build Calculator

A CLI tool for calculating and optimizing Elder Scrolls Online builds.

## Architecture

This project follows a Domain-Driven Design (DDD) approach adapted for a CLI application with static data.

### Layers

```
src/
├── cli/           # Entry points for commands
├── data/          # Static data (replaces database)
├── domain/        # Core domain entities and value objects
├── services/      # Orchestration layer
└── infrastructure/# Cross-cutting concerns
```

#### `cli/` - Command Entry Points
Entry points for CLI commands using `clap`. Each command (e.g., `optimize`, `view`) has its own module that parses arguments and delegates to services.

#### `data/` - Static Data Layer
Replaces a traditional database. Contains hardcoded game data:
- `skills/` - Skill definitions by class and weapon type
- `passives/` - Passive abilities by class and weapon type
- `bonuses/` - Champion points, unique bonuses

Data is exposed as static references (`&'static`) via `Lazy` initialization.

#### `domain/` - Domain Layer
Core business entities and value objects. Contains no external dependencies. Key types:
- `SkillData`, `PassiveData`, `BonusData` - Core entities
- `Build` - Aggregate representing a character build
- Value objects: `DamageType`, `Resource`, `WeaponType`, `ClassName`, etc.

#### `services/` - Orchestration Layer
Coordinates multiple domain entities and data sources. Services handle:
- `SkillsService` - Skill filtering, morph selection
- `PassivesService` - Passive resolution
- `BuildOptimizer` - Build optimization algorithms

#### `infrastructure/` - Cross-Cutting Concerns
Utilities shared across layers:
- `logger` - Console output formatting
- `format` - Number/duration formatting
- `table` - Table rendering
- `combinatorics` - Combination/permutation helpers

## Development

```bash
cargo build
cargo test
cargo run -- optimize --help
cargo run -- view --help
```
