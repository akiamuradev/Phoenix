# Contributing to Phoenix

Phoenix is early-stage software. Small, focused changes that preserve its architectural boundaries are the most useful contributions.

## Workflow

1. Synchronize `main` with `origin/main`.
2. Create a concise branch for one logical change.
3. Implement and validate the change.
4. Commit with a clear Conventional Commit-style message.
5. Push the branch and open a pull request targeting `main`.
6. Merge only after required checks succeed, then delete the branch.

Do not commit directly to `main`, generated build output, `.phx` workspaces, caches, credentials, or editor state.

## Quality checks

Run the checks relevant to your change; before a full pull request, run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
npm ci
npm run format:check
npm run typecheck
npm run lint
npm run build
```

Platform-specific desktop work should also be checked on Linux, Windows, and macOS where the Tauri toolchain permits.

## Architecture

- Keep domain rules and persistence in `phoenix-core`; it must not depend on Tauri.
- Treat SQLite workspace data as authoritative and every canvas as a view.
- Model relationship types as extensible data.
- Keep source-provider details behind adapter boundaries.
- Add an explicit migration for every schema change and test reopening persisted data.

By contributing, you agree that your contribution is licensed under GPL-3.0-or-later.
