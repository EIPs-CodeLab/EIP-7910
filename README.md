# EIP-7910 eth_config for reth

Implementation skeleton for exposing the `eth_config` JSON-RPC method (EIP-7910) as a reth Execution Extension / RPC namespace.

## What this project provides
- Workspace layout with three crates:
  - `eth-config-api`: pure data model and helpers (no reth deps).
  - `eth-config-reth`: reth integration layer (feature-gated) with resolver/cache/RPC stubs.
  - `eth-config-cli`: optional CLI to diff `eth_config` responses across nodes.
- Example JSON payloads under `examples/` that mirror the EIP response shape.
- Makefile shortcuts for fmt/lint/check/test/doc and CLI build.

## Current status (2026-03-10)
- Types, module structure, and placeholder resolvers are in place.
- Integration logic still needs to derive fork data from `ChainSpec`, compute FORK_HASH (EIP-6122), and wire jsonrpsee modules into reth.
- Golden tests are stubbed and marked `#[ignore]`; they need real fixtures from the EIP.

## Building & dev commands
- `make fmt` — format workspace.
- `make lint` — clippy all targets; set `FEATURES=reth` to include reth integration.
- `make check` / `make test` — fast correctness checks (tests currently placeholder).
- `make doc` — build docs without deps.
- `make cli` — build the validator CLI.

## Tests
- `tests/hoodi_prague.rs`: golden comparison versus the EIP Prague sample output (fixture pending).
- `tests/hoodi_cancun.rs`: golden comparison versus the EIP Cancun sample output (fixture pending).
- `tests/no_future_fork.rs`: ensures `next`/`last` are null when no future or past fork exists.
- `tests/bpo_fork.rs`: covers blob-parameter-only fork transitions.
- Tests live in the `eth-config-tests` crate (workspace member) and currently validate the example fixtures deserialize correctly. Run them via `make test` or `cargo test -p eth-config-tests`.

## EIP-7910 recap
- Adds the `eth_config` RPC method to surface the active fork, next/last fork, precompile set, system contracts, blob schedule, and FORK_HASH.
- Response shape (camelCase):
  - `current`, `next`, `last`: fork entries containing fork config (name, block, blobSchedule?, precompiles[], systemContracts[]).
  - `forkId`: `{ hash, next? }` matching EIP-6122 semantics.
- Caching requirement: responses must invalidate when crossing a fork boundary.

## Repository layout
- `crates/eth-config-api/src`: data models (`types.rs`), FORK_HASH placeholder (`fork_id.rs`), registries for precompiles/system contracts.
- `crates/eth-config-reth/src`: resolver/cache/RPC/namespace stubs (behind `reth` feature).
- `crates/eth-config-cli/src`: clap-based placeholder binary.
- `tests/`: integration test scaffolding for EIP sample outputs and edge cases.
- `examples/`: sample `eth_config` payloads for Cancun, Prague, and blob-parameter-only scenarios.

## How to finish the implementation
- Implement `ForkConfigResolver` to read `reth::chain_spec::ChainSpec` and emit `ForkConfig` for current/next/last at a given block/slot.
- Compute `ForkId` per EIP-6122 (hash over fork blocks) in `fork_id.rs`.
- Populate precompile/system-contract registries per fork and thread them through the resolver.
- Wire `EthConfigApi` into reth’s RPC builder (`namespace.rs`) using jsonrpsee.
- Fill the golden JSON fixtures from the EIP text and enable the ignored tests.

## Notes
- The crate names contain hyphens; the Rust module imports use underscores (e.g., `eth_config_api`).
- The reth integration is feature-gated to avoid pulling the reth dependency unless requested.
