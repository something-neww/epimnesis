# Agent Development Guide

This file provides coding standards, build commands, and conventions for agents working in this repository.

## Project Structure

Mixed-language monorepo: `packages/` (TypeScript/JS), `services/` (Go), `apps/` (future apps). Uses Turborepo for orchestration across all packages.

## Build, Lint, Format, Test Commands

### Root-level (Turborepo)

```bash
pnpm build        # Build all packages and services
pnpm dev          # Dev mode for all packages and services
pnpm lint         # Lint all packages and services
pnpm format       # Format all code
pnpm clean        # Clean build artifacts
```

### TypeScript/JavaScript (packages/)

```bash
cd packages/ts-sdk
pnpm build        # tsdown
pnpm dev          # tsdown --watch
pnpm lint         # oxlint . --type-aware
pnpm format       # oxfmt --write
```

### Go (services/)

```bash
cd services/core
go build ./...    # Build
golangci-lint run # Lint (strict)
goimports -w . && golines -w -l 80 . # Format
go test ./...     # All tests
go test -run TestFunc ./path/to/pkg # Single test
go test -v ./... # Verbose
go test -race ./... # Race detection
```

## TypeScript Code Style

**Formatting**: oxfmt, 80 char max, auto-import sorting

**Import Order**: side-effect → builtin → external → internal → parent → sibling → index

**Naming**: filenames kebab-case, vars camelCase, constants UPPER_SNAKE_CASE, types/interfaces PascalCase, functions camelCase

**Strict Rules**: NO `any` types, no type assertions without checks, remove unused imports, separate type imports (`import type { }`), use `Type[]` not `Array<Type>`, throw Error objects only, prefer `const` over `let`

**Error Handling**: handle promise rejections, try/catch async, prefer async/await

**Organization**: package exports in `index.ts`/`index.mts`, ESM modules, build artifacts in `dist/`

## Go Code Style

**Formatting**: golangci-lint (goimports, golines, gofumpt), 80 char max

**Import Order**: std lib → third-party → local (github.com/epimnesis/*)

**Naming**: packages lowercase single word, interfaces PascalCase (`Store`), structs PascalCase (`PostgresStore`), exported funcs/methods PascalCase, unexported camelCase, vars camelCase, exported constants PascalCase, unexported camelCase, errors prefix `Err`, suffix `Error` for types

**Error Handling**: ALWAYS check errors, return don't panic, wrap with `fmt.Errorf("context: %w", err)`, early returns, no nil returns with error

**Dependency Injection (Uber FX)**: Use `go.uber.org/fx`, export factories in `package.Module` fx.Options, unexported constructors (`new*`) returning interfaces, `fx.Provide()` to register, `fx.Invoke()` for startup

**Architecture**: Interface-first design, implementations use unexported structs, domain layers (entity, repository, infrastructure), use `context.Context`, struct tags `json:"field"`, `env:"ENV_VAR"`

**Prohibited**: NO `init()` (use fx.Provide), NO globals (use DI), NO embedded sync.Mutex (use composition), use `log/slog` not `log`, use `math/rand/v2` not `math/rand`, use `google.golang.org/protobuf` not `github.com/golang/protobuf`

**Testing**: testify assertions, `_test` packages where appropriate, `TestFunctionName(t *testing.T)`, use `t.Parallel()`, table-driven tests

**File Organization**: `cmd/main.go` entry points, `internal/` private code (domain, repository, infrastructure, interfaces), one dir = one package, go.work for workspace

## Common Pitfalls

**TypeScript**: No `@ts-ignore`/`@ts-expect-error` without justification, no `any`, remove unused imports, keep type/value imports separate

**Go**: Always handle errors, no empty structs `{}`, no `//nolint` without explanation, no nil returns with error

## Linting

TypeScript: oxlint (fast, strict), Go: golangci-lint (80+ linters), both configured at root

## Version Management

pnpm for JS/TS, go modules for Go, Changesets for TS versioning, conventional commits (feat:, fix:, refactor:)
