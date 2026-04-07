# xlm-ns

`xlm-ns` is a Rust workspace for a Stellar name service where names like
`timmy.xlm` behave as user-owned identifiers for accounts, apps, subdomains, and
cross-chain resolution targets.

The repository is organized as a multi-crate system so the core naming logic can
be tested locally before it is wired into Soroban-specific storage, auth, and
deployment flows.

## Vision

The target user experience is straightforward:

- A user registers a base name such as `timmy.xlm`.
- That name resolves to a Stellar address or another delivery target.
- The owner can update resolver data, renew the registration, transfer ownership,
  create subdomains, or bridge the name to external resolver networks.
- Premium names can be sold through auctions instead of first-come-first-served
  issuance.

## Current status

The workspace now contains real contract-domain logic instead of only placeholder
stubs:

- Shared validation for labels, full names, registration periods, owners, and
  chain identifiers.
- Lifecycle-aware name records with registration, expiry, and grace-period data.
- Stateful registry, registrar, resolver, auction, subdomain, NFT, and bridge
  contract logic.
- Unit tests for all contract crates covering the main happy-path flows.

What is not implemented yet:

- Soroban storage primitives and entrypoint macros.
- On-chain authorization using Stellar accounts and signatures.
- Real payment settlement, token transfers, and chain messaging.
- Production deployment scripts and end-to-end integration tests.

## Workspace layout

### Contracts

- `contracts/registry`
  Purpose: canonical name ownership state.
  Responsibilities:
  - Stores `NameRecord` ownership and metadata.
  - Enforces active/grace/claimable lifecycle checks.
  - Restricts mutation to the current owner.
  - Supports transfer, resolver updates, target updates, metadata updates, and
    expiry extension.

- `contracts/resolver`
  Purpose: forward and reverse resolution.
  Responsibilities:
  - Maps `name -> resolution record`.
  - Maps `address -> primary name`.
  - Stores bounded text records such as social handles or app metadata.
  - Enforces owner-controlled updates and deletion.

- `contracts/registrar`
  Purpose: registration issuance and renewal policy.
  Responsibilities:
  - Computes quotes from label length and registration duration.
  - Tracks reserved names.
  - Accepts registrations and renewals.
  - Maintains treasury balance accounting in the domain model.
  - Uses explicit expiry and grace-period rules.

- `contracts/auction`
  Purpose: premium-name sale flow.
  Responsibilities:
  - Creates auctions with a reserve price and bidding window.
  - Records bids with timestamps.
  - Settles using a Vickrey-style second-price outcome.
  - Supports unsold outcomes when the reserve is not met.

- `contracts/subdomain`
  Purpose: delegated namespace management.
  Responsibilities:
  - Registers parent domains for subdomain issuance.
  - Supports parent owners and delegated controllers.
  - Creates and transfers owned subdomains such as `pay.timmy.xlm`.

- `contracts/nft`
  Purpose: tokenized representation of name ownership.
  Responsibilities:
  - Mints ownership tokens.
  - Tracks owner, approval, and metadata.
  - Supports approval-based transfers.

- `contracts/bridge`
  Purpose: cross-chain resolution payload construction.
  Responsibilities:
  - Registers supported destination chains.
  - Maps chains to resolver and gateway targets.
  - Builds deterministic Axelar-style payloads for resolution propagation.

### Packages

- `packages/xlm-ns-common`
  Shared constants, errors, types, and validation helpers used by the contract
  crates.

- `packages/xlm-ns-sdk`
  A lightweight Rust SDK surface for future wallet and dapp integration.

### Operator tooling

- `cli/`
  Simple command-line entry points for register, resolve, renew, transfer, and
  auction flows.

- `scripts/`
  Shell helpers for deploy, invoke, and local setup tasks.

- `tests/`
  Placeholders for integration scenarios and test fixtures shared across crates.

## Core domain model

`NameRecord` in `packages/xlm-ns-common` is the shared type used by the main
contract flows. It currently tracks:

- `label`
- `tld`
- `owner`
- `resolver`
- `target_address`
- `ttl_seconds`
- `registered_at`
- `expires_at`
- `grace_period_ends_at`

This matters because the registry and registrar now reason about the same
registration lifecycle:

- Active: `now <= expires_at`
- Grace period: `expires_at < now <= grace_period_ends_at`
- Claimable by a new owner: `now > grace_period_ends_at`

## Registration flow

The intended contract interaction order is:

1. Ask the registrar for a quote using the requested label and registration
   duration.
2. Submit payment and create a registration record.
3. Materialize the name in the registry with the resulting ownership state.
4. Set resolver records for forward and reverse lookups.
5. Optionally mint an NFT and configure bridge routes or subdomains.

The current codebase models each of those steps, but not yet as a single
integrated on-chain transaction graph.

## Naming and validation rules

Shared validation currently enforces:

- Minimum and maximum label length.
- Lowercase ASCII letters, digits, and hyphens only.
- No leading or trailing hyphen.
- Explicit `.xlm` TLD parsing for base names.
- Bounded registration durations.
- Non-empty owner and chain identifiers.

## Local development

Format the workspace:

```sh
cargo fmt --all
```

Run tests:

```sh
TMPDIR=/tmp cargo test --workspace
```

`TMPDIR=/tmp` is used here because the current sandbox environment does not allow
Rust to create temporary build directories in the default macOS temp location.

## Recommended next steps

The highest-value contract work from here is:

1. Replace the in-memory Rust maps with Soroban storage accessors.
2. Introduce real auth boundaries for owner-only and controller-only actions.
3. Connect registrar success paths directly into registry writes.
4. Add end-to-end tests that exercise registrar, registry, resolver, NFT, and
   subdomain flows together.
5. Replace placeholder bridge payload routing with actual Axelar/Stellar message
   integration.
