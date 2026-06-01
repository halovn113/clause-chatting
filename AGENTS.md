# AGENTS.md — Claude Code Leaked Source

## Repo nature

This is a **read-only mirror** of the leaked Claude Code (Anthropic's AI coding CLI) source code. It contains no build config, no package.json, no lockfile, no test infrastructure, no CI, no linter/formatter/typecheck config. Everything lives under `src/`.

## Entrypoints

- `src/main.tsx` — CLI entrypoint (Commander.js + React/Ink). Side-effect imports at the top for startup profiling, MDM config reads, and keychain prefetch — do not reorder.
- `src/setup.ts` — Session initialization (config loading, analytics, project root detection).

## Architecture

- **React/Ink** terminal renderer — custom copy lives in `src/ink/` (forked/rewritten).
- **40+ tools** in `src/tools/` — BashTool, Files, LSP integration, Web fetching, etc.
- **Multi-agent orchestration** in `src/coordinator/` (Swarm-based).
- **LLM core** in `src/QueryEngine.ts`.
- **Services** in `src/services/`: MCP servers, OAuth, analytics, session memory, autoDream, voice.
- **Buddy/Tamagotchi** system in `src/buddy/`.
- **IDE bridge** in `src/bridge/`.
- **Migrations** in `src/migrations/` — settings migration scripts run on upgrade.

## Build tool

Uses **Bun** (`bun:bundle` feature flag imports). Not buildable with Node.js alone.

## No tests, no verification

This repo has zero tests, zero CI, zero lint/typecheck. Do not run or expect any.

## Key conventions

- Import paths use `src/...` absolute-style (Bun resolves them).
- `custom-rules/no-top-level-side-effects` ESLint rule forbids top-level side effects (enforced in the original build).
- Types in `src/types/`; branded types like `SessionId` in `src/types/ids.ts`.
- Startup performance is critical — the code aggressively parallelizes initialization (keychain reads, MDM queries, etc.).
- Settings/config are read from `src/utils/config.ts`; environment from `src/utils/env.ts`.
