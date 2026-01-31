# Agent Guidelines for Epimnesis

This document guides agentic coding assistants working in the Epimnesis monorepo.

## Build / Lint / Test Commands

### Root-level
```bash
pnpm run dev build test format lint check-types
```

### Single test
- Rust: `cargo test <name>` (in `crates/engine/` or `bindings/node/`)
- TypeScript Vitest: `vitest run <file>` or `vitest -t "<name>"`
- TypeScript Ava: `ava <file>` or `ava -t "<name>"`

### Language-specific
- **Rust**: `cargo fmt test`, `taplo format`
- **TypeScript**: `oxlint . --type-aware`, `oxfmt --write`
- **Node bindings**: `napi build --platform`, `ava`

## Testing Infrastructure

### Frameworks
- **Ava**: TypeScript/Node bindings (bindings/node/)
- **Vitest**: TypeScript SDK (packages/ts-sdk/)
- **Cargo test**: Rust engine (crates/engine/)

### Config & Conventions
- **TypeScript**: `__test__/` (double underscore, singular), embedded in package.json
- **Rust**: `tests/` directory, default cargo config
- **Ava**: Embedded in package.json, 2m timeout, worker threads disabled
- **Vitest**: No explicit config
- **Mock patterns**: Pure unit tests only, no mock libraries, inline data
- **Integration**: Unit tests only, self-contained, no fixtures/helpers

### Mock Patterns
- **Pure unit tests only** - No mock libraries (sinon, jest.mock, nock)
- Direct function calls with inline test data
- No external API mocking (network, filesystem, databases)
- Minimal test setup - test data defined inline
- Self-contained tests - each file independent

### Testing Philosophy
- **Pure function testing**: Isolated, no side effects
- **No test fixtures/helpers**: Avoid shared utilities
- **Inline test data**: All test data in test files
- **Self-contained**: Each test file runs independently

### Commands
- **Root**: `pnpm run test` (turbo-orchestrated)
- **Per package**: bindings/node→`ava`, ts-sdk→`vitest`, engine→`cargo test`
- **Direct**: `cargo test <name>`, `vitest -t "<name>"`, `ava -t "<name>"`

## Code Style Guidelines

### TypeScript

**Formatting & Linting**
- Use **oxlint** for linting (`.oxlintrc.json`)
- Use **oxfmt** for formatting (80-char line width)
- Imports sorted: side-effect, builtin, external, internal, parent, sibling, index

**Features & Conventions**
- TypeScript 5.x with strict type checking
- `type` for aliases, `interface` for shapes, discriminated unions for variants
- `camelCase` for variables/functions, `PascalCase` for classes/types
- `SCREAMING_SNAKE_CASE` for constants, `#fieldName` for private fields
- `const _exhaustive: never = value` for exhaustive switch cases
- `export type` for type-only exports
- Throw `Error` with descriptive messages

### Rust

**Formatting & Style**
- Standard `cargo fmt` (4-space indentation)
- `taplo format` for TOML files
- `pub mod modname;` at top, then `use crate::modname::*;`

**Features & Conventions**
- `snake_case` for functions/variables, `PascalCase` for types/structs/enums
- `SCREAMING_SNAKE_CASE` for constants
- Implement `From<T>`/`Into<U>` for FFI conversions
- Use `Option<T>` instead of nullable types
- `HashMap<K, V>` for metadata/dictionaries
- Convert `f32` (Rust) → `f64` (NAPI/JS) explicitly with `as`
- Exhaustive `match` with `_` fallback, prefer `if let`
- Use `debug_assert!` for invariants in pure functions

**FFI & Documentation**
- Use `#[napi]`/`#[napi(object)]`/`#[napi(string_enum)]` derive macros
- Document all public APIs with `///`
- Include architectural notes with `/// NOTE:` or `/// IMPORTANT:`

## Anti-Patterns

### NEVER Patterns
- **Engine must NEVER panic** (`crates/engine/src/scoring.rs:128`) - Invalid cosine similarity returns 0.0 instead of crashing
- **Functions must be deterministic** (`crates/engine/src/scoring.rs:182`) - No randomness or external state in scoring

### MUST Constraints
- **Engine MUST be stateless** (`crates/engine/src/lib.rs:2`) - No I/O, storage, embeddings, or orchestration
- **Procedural activation MUST be deterministic** (`crates/engine/src/scoring.rs:181`) - No external state or randomness
- **FFI contract MUST remain stable** (`bindings/node/src/lib.rs:2`) - Major version bumps required for breaking changes

### TODO Items (Future Work)
- **Post-v0**: Move weights to SDK with presets (`crates/engine/src/weights.rs:36`) - Allow presets (e.g., "chat", "agent", "research")
- **Architecture migration**: Scoring weights and decay parameters move to SDK layer

### IMPORTANT Patterns
- Keep public API minimal in v0.x, avoid premature configuration (`crates/engine/src/weights.rs:3`)
- Serve as stable, opinionated baseline until real-world usage validates knobs
- Trigger matching remains simple (first matching trigger wins)
- Weights are DEFAULT heuristics, not permanent configuration

### NOTE Patterns
- Stateless memory reasoning engine - pure scoring and ranking only (`crates/engine/src/lib.rs:1`)
- Default weights not for long-term tuning - exist for usability (`crates/engine/src/weights.rs:32`)
- NAPI output types treated as read-only truth
- Engine types are authoritative source of truth

### Architectural Invariants

**Separation of Concerns**
1. **Engine (`crates/engine/`)**: Pure, stateless scoring and ranking
   - NO I/O
   - NO storage
   - NO infrastructure
   - NO external state

2. **Node bindings (`bindings/node/`)**: FFI layer only
   - Stable contract (major version bumps for changes)
   - Type conversion (Rust → JS)
   - No business logic

3. **SDK (`packages/ts-sdk/`)**: User-facing API
   - Configuration and orchestration
   - Mappers for type conversions
   - Infrastructure integration

**Engine Purity**
- All candidates pre-filtered by TypeScript SDK
- Engine responsible ONLY for: scoring, ranking, explainability
- If memory cannot explain why it scored, must not participate in ranking
- At least one scoring signal must exist for each candidate

**Safety Over Purity**
- Engine functions never panic in production code paths
- Defensive programming for invalid inputs
- Safe defaults over mathematical correctness
- Deterministic ordering (score then ID) for stability

### Anti-Patterns to Prevent
- **DO NOT put I/O in engine** - Engine is pure, stateless, no storage
- **DO NOT make engine stateful** - All state managed by SDK
- **DO NOT break FFI contract** - Major version bumps required
- **DO NOT prematurely configure** - Keep API minimal in v0.x
- **DO NOT panic in production** - Safe defaults over mathematical purity

## Monorepo Structure

### Workspace Configuration
- **Non-standard**: `crates/engine` and `bindings/node` in pnpm workspace
- **Purpose**: Turbo orchestration for cargo fmt/test commands
- **Dual workspace**: Rust workspace (Cargo.toml) + npm workspace (pnpm-workspace.yaml)

### Package Naming Issues
- `crates/engine/package.json` → "name": "engine" (unscoped, internal-only)
- `packages/ts-sdk/package.json` → "name": "@epimnesis/core" (mismatched: directory=ts-sdk, name=core)

### Workspace Package Inventory
- `bindings/node` - NAPI-RS FFI bindings
- `crates/engine` - Pure Rust engine (Turbo shim only)
- `docs` - Documentation site
- `examples` - Example implementations
- `packages/*` - npm packages (ts-sdk, config)
- `tooling/*` - Internal tooling

**Rust workspace members (Cargo.toml):**
- `crates/engine` - Engine crate
- `bindings/node` - FFI binding crate

### Architecture Layout (3-Layer Separation)
1. **Engine (`crates/engine/`)**: Pure Rust memory reasoning engine
   - Stateless scoring and ranking
   - No I/O, no storage, no infrastructure

2. **Node bindings (`bindings/node/`)**: NAPI-RS FFI layer
   - Converts Rust types to JS types
   - Stable public API contract
   - Builds native modules for Windows, macOS (x64/ARM64), Linux

3. **SDK (`packages/ts-sdk/`)**: TypeScript user-facing API
   - Orchestration and configuration
   - Mappers for type conversions
   - Infrastructure integration

### Hybrid Packages
- **crates/engine**: Rust crate with npm shim (package.json hooks for turbo)
- **bindings/node**: Rust FFI crate with npm package (@epimnesis/node)

### Deviations from Standard Patterns
- `bindings/` at root (not in `packages/`)
- Unscoped `engine` package name
- Mismatched `ts-sdk` directory vs `core` package name
- Dual root manifests (package.json + Cargo.toml)

### Standard vs Custom Patterns
- **Standard patterns**: `packages/` for npm packages, scoped package names (`@epimnesis/*`)
- **Custom/deviant patterns**: Rust crates in pnpm workspace, `bindings/` at root

### Why This Structure Works

**Turbo orchestration unifies ecosystems**
- Single command (`pnpm run test`) runs all tests
- Cargo tests via pnpm scripts in Rust crates
- npm tests (Vitest/Ava) via pnpm scripts
- No manual ecosystem switching

**Hybrid packages enable cross-language workspaces**
- Rust crates participate in npm workspace via package.json shim
- TypeScript packages depend on Rust via `workspace:*`
- pnpm hoisting works across language boundaries

**FFI layer creates clear boundary**
- Rust engine isolated from TypeScript concerns
- NAPI-RS provides stable ABI between layers
- Each layer can evolve independently (with coordination)

## Conventions & Build/CI

### Tooling Stack

**TypeScript Tooling**
- **Oxlint** (`oxlint`): Fast linting with plugins
  - `unicorn`: Modern JavaScript best practices
  - `typescript`: TypeScript-specific rules
  - `oxc`: Ox compiler optimizations and safety checks
- **Oxfmt** (`oxfmt`): Fast formatting
  - 80-character line width limit
  - Import sorting with blank line groups
  - External dependencies first, internal imports after

**Rust Tooling**
- **Cargo fmt**: Standard Rust formatting (4-space indentation)
- **Taplo** (`taplo format`): TOML file formatting

**Monorepo Orchestration**
- **Turborepo**: Build task orchestration and caching
  - UI: tui (terminal user interface)
  - Task dependencies: `^build`, `^test`, `^lint`, etc.
  - Cache: disabled for `dev` task, enabled for others
  - Outputs tracking: `dist/**` for build tasks

**Git Hooks**
- **Lefthook**: Pre-commit hook management
  - Parallel execution of hooks
  - Automatic staging of fixed files (`stage_fixed: true`)
  - Runs on all staged files

### Linting Configuration (.oxlintrc.json)

**Rule Severity**
- All rules set to `"warn"` (no blocking errors)
- Comprehensive coverage across three plugin sets

**Linting Categories**
- Core ESLint compatibility (constructor-super, no-async-promise-executor, etc.)
- OXC optimizations (bad-array-method-on-arguments, const-comparisons, etc.)
- TypeScript safety (await-thenable, no-floating-promises, no-array-delete, etc.)
- Unicorn best practices (no-empty-file, prefer-set-size, prefer-string-starts-ends-with, etc.)

**Linting Commands**
- Root: `pnpm run lint` → `turbo lint`
- Package: `pnpm run lint` → `oxlint . --type-aware`
- Pre-commit: `lefthook` → `pnpm oxlint --fix {staged_files}`

### Formatting Conventions

**Line Width**
- 80-character hard limit for TypeScript (enforced by oxfmt)
- Automatic wrapping and line breaks

**Import Ordering**
1. External dependencies (e.g., `@epimnesis/node`)
2. Blank line separator
3. Internal relative imports (e.g., `./ffi`, `../types`)
4. Type imports: `import type { }` grouped separately

**Formatting Commands**
- Root: `pnpm run format` → `turbo format`
- TypeScript: `oxfmt --write {files}`
- Rust: `cargo fmt`
- TOML: `taplo format`

### TypeScript Strictness (@epimnesis/config/tsconfig.base.json)

**Base Strict Settings**
- `"strict": true`: All strict type-checking options enabled
- `"verbatimModuleSyntax": true`: Explicit import/export syntax enforcement

**Additional Strictness**
- `"noUncheckedIndexedAccess": true`: Array access returns `T | undefined`
- `"noUnusedLocals": true`: Warn on unused local variables
- `"noUnusedParameters": true`: Warn on unused function parameters
- `"noFallthroughCasesInSwitch": true`: Exhaustive switch cases required

**Module Configuration**
- `"moduleResolution": "bundler"`: Bundler-friendly resolution
- `"target": "ESNext"`, `"lib": ["ESNext"]`: Latest JS features
- `"isolatedModules": true`: Each file can be transpiled independently

### Git Hooks (lefthook.yml)

**Pre-commit Hook Behavior**
- Parallel execution: oxlint and oxfmt run simultaneously
- Auto-fix mode: `--fix` and `--write` flags
- Auto-stage: `stage_fixed: true` commits fixes automatically
- Staged files only: `{staged_files}` token

### Lint-staged Configuration

**Root package.json**: Runs oxlint and oxfmt on all staged files
**Rust crates (crates/engine/package.json)**: Runs taplo format on all staged TOML files
**Behavior**: Integrates with lefthook for consistent execution

### Build System (turbo.json)

**Task Dependencies**
- `build`: Depends on `^build` (all upstream builds)
- `test`: Depends on `^test` (all upstream tests)
- `format`: Depends on `^format` (all upstream formats)
- `lint`: Depends on `^lint` (all upstream lints)
- `check-types`: Depends on `^check-types` (all upstream type checks)

**Task Configuration**
- `build`: Tracks `.env*` inputs and `dist/**` outputs
- `dev`: `cache: false`, `persistent: true` (no caching, long-running)
- Other tasks: Default cache behavior

**Root Scripts (package.json)**
- `"dev": "turbo dev"`
- `"build": "turbo build"`
- `"test": "turbo test"`
- `"format": "turbo format"`
- `"lint": "turbo lint"`
- `"check-types": "turbo check-types"`

### Package-Specific Conventions

**TypeScript packages (packages/ts-sdk)**
- `oxlint . --type-aware`: Type-aware linting
- `oxfmt --write`: Format all files
- `vitest`: Test framework

**Rust crates (crates/engine)**
- `cargo fmt`: Format Rust code
- `taplo format`: Format TOML files
- `cargo test`: Run Rust tests

**Hybrid packages (bindings/node)**
- Both npm scripts (`ava` for tests) and Cargo commands
- NAPI-RS build process for native modules

### Key Project Conventions Summary

**Linting**
- Fast oxlint with comprehensive rule set (unicorn, typescript, oxc)
- All warnings, no blocking errors
- Type-aware mode enabled

**Formatting**
- 80-char line width (oxfmt)
- Import sorting with blank line groups
- Rust: cargo fmt, TOML: taplo format

**TypeScript**
- Strict mode enabled
- Additional strictness: noUncheckedIndexedAccess, noUnusedLocals/Parameters
- verbatimModuleSyntax for explicit imports

**Git Hooks**
- Lefthook manages pre-commit hooks
- Parallel execution: oxlint + oxfmt
- Auto-fix and auto-stage enabled

**Build System**
- Turborepo orchestrates monorepo tasks
- Dependency graphs: `^build`, `^test`, etc.
- Cache enabled (except dev)

**Hybrid Ecosystem**
- Rust crates in pnpm workspace (via package.json shims)
- Dual workspace: Cargo.toml + pnpm-workspace.yaml
- Single command (`pnpm run test`) runs all tests across ecosystems

## Architectural Principles

### Separation of Concerns
- **Engine (`crates/engine/`)**: Pure, stateless scoring and ranking logic. No I/O, no storage, no infrastructure.
- **Node bindings (`bindings/node/`)**: FFI layer using napi-rs. Converts Rust types to JS types.
- **SDK (`packages/ts-sdk/`)**: User-facing API, configuration, orchestration, and mappers.

### Code Organization
- Use mapper pattern for type conversions (e.g., `CandidateMapper.toJs()`)
- Discriminated unions for memory types with kind field
- Prefer composition over inheritance
- Keep functions small and pure when possible

### Dependencies
- **TypeScript**: Use workspace dependencies (`@epimnesis/node`, `@epimnesis/config`)
- **Rust**: Use workspace dependencies (`engine`)
- No external runtime dependencies in the engine crate

### Testing
- Rust tests use built-in `cargo test`
- TypeScript tests use Vitest (SDK) or Ava (Node bindings)
- Tests are located in `__test__` directories (Ava) or alongside source files (Vitest/Rust)

### Commit Guidelines
- Pre-commit hooks run oxlint and oxfmt automatically (via lefthook)
- Hooks run in parallel: `oxlint --fix` and `oxfmt --write` simultaneously
- Auto-stage enabled: Fixed files are staged automatically (`stage_fixed: true`)
- Never commit without running `pnpm run lint` and `pnpm run format`
- Lefthook manages Git hooks (configured in `lefthook.yml`)

## Package Manager
- **pnpm** is the package manager (v10.26.2+)
- Use `pnpm install` for dependencies
- Use `pnpm workspace:*` for internal dependencies in package.json

## Entry Points

### TypeScript Entry Points
- **Primary SDK Entry**: `packages/ts-sdk/src/index.ts` (Epimnesis class, retrieve/recall methods)
- **FFI Bindings Entry**: `packages/ts-sdk/src/ffi/index.ts` (mappers: BaseMemoryMapper, CandidateMapper, RetrieveMapper)

### Rust Entry Points
- **Node FFI Contract**: `bindings/node/src/lib.rs` (JsMemoryEngine struct with napi-derive)
- **Core Engine Library**: `crates/engine/src/lib.rs` (MemoryEngine struct, stateless)

### Package.json Entry Points

**Root package.json**
- Private monorepo, development only
- No main/bin fields

**@epimnesis/core (ts-sdk)**
- main: `./dist/index.mjs`
- module: `./dist/index.mjs`
- types: `./dist/index.d.mts`
- exports: `{ ".": "./dist/index.mjs", "./package.json": "./package.json" }`

**@epimnesis/node (Rust bindings)**
- main: `index.js`
- browser: `browser.js`

**engine (Rust crate)**
- Library crate only, not published to npm
- No main/bin fields

### Architecture Flow
```
User Code (@epimnesis/core)
    ↓ imports
FFI Bindings (@epimnesis/node)
    ↓ FFI (napi-rs)
Core Engine (engine crate - stateless)
```

**Layer Responsibilities**
- **@epimnesis/core**: User-facing API, orchestration, configuration
- **@epimnesis/node**: FFI bridge, type conversion (Rust → JS), stable ABI
- **engine crate**: Pure scoring/ranking, stateless, no infrastructure

### Binary Targets

**None** - Pure library project
- No CLI binaries
- No executable entry points
- Library-first design for integration

### Key Characteristics
- **Library-first architecture** - Designed for integration, not standalone execution
- **No binary targets** - Pure library project with no CLI entry points

## File Extensions
- TypeScript: `.ts`
- Rust: `.rs`
- Configuration: `.json`, `.toml`, `.yml`
