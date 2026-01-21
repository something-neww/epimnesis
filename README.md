# epimnesis

## Features

- **TypeScript** - For type safety and improved developer experience
- **Oxlint** - Oxlint + Oxfmt (linting & formatting)
- **Turborepo** - Optimized monorepo build system

## Getting Started

First, install the dependencies:

```bash
pnpm install
```

Then, run the development server:

```bash
pnpm run dev
```

## Git Hooks and Formatting

- Initialize hooks: `pnpm run prepare`
- Format and lint fix: `pnpm run check`

## Project Structure

```
epimnesis/
├── apps/
```

## Available Scripts

- `pnpm run dev`: Start all applications in development mode
- `pnpm run build`: Build all applications
- `pnpm run dev:web`: Start only the web application
- `pnpm run check-types`: Check TypeScript types across all apps
- `pnpm run check`: Run Oxlint and Oxfmt
