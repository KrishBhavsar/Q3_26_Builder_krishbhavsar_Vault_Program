# Q3 26 Vault

A Solana vault program built with [Anchor](https://www.anchor-lang.com/) and Rust. Each user gets an individual PDA-backed vault where they can deposit and withdraw lamports. The vault can also be closed to recover its remaining balance and state-account rent.

## Features

- Initialize one vault per user.
- Deposit lamports into the user's vault.
- Withdraw lamports while preserving the vault's rent-exempt reserve.
- Close the vault and return its remaining lamports to the user.
- Validate that deposit and withdrawal amounts are greater than zero.
- Integration-test the complete lifecycle with [LiteSVM](https://github.com/LiteSVM/litesvm).

## Program details

- Program ID: `9dwzKut333e2LuYyidfg7BQDp74kVsuGZKDRCSx6woZA`
- Network configured in `Anchor.toml`: `localnet`
- Rust version: `1.89.0`
- Anchor dependency: `1.1.2`

## Account model

The program derives two accounts for each user:

| Account | Seeds | Purpose |
| --- | --- | --- |
| `vault_state` | `["state", user_pubkey]` | Stores the vault and state PDA bumps. |
| `vault` | `["vault", user_pubkey]` | System-owned PDA that holds the user's lamports. |

The vault is funded with the rent-exempt minimum during initialization. Withdrawals may only use the balance above that reserve. Closing the vault transfers its entire balance to the user and closes `vault_state`.

## Instructions

### `initialize`

Creates the user's `vault_state` account and derives the user's vault PDA. The user pays for the state account and the vault's rent-exempt minimum balance.

### `deposit(amount)`

Transfers `amount` lamports from the user to the user's vault. The amount must be greater than zero.

### `withdraw(amount)`

Transfers `amount` lamports from the vault to the user. The amount must be greater than zero and cannot exceed the vault balance after its rent-exempt reserve is excluded.

### `close`

Closes `vault_state`, transfers the vault's complete lamport balance to the user, and leaves the vault empty.

## Prerequisites

Install the following tools:

- Rust `1.89.0` (the repository includes `rust-toolchain.toml`)
- Solana CLI
- Anchor CLI compatible with the project's Anchor version

Configure a local Solana wallet if needed:

```bash
solana config set --url localhost
solana-keygen new
```

## Build

From the repository root, run:

```bash
cargo build
```

To build the Solana program binary with Anchor:

```bash
anchor build
```

## Test

The integration test loads the compiled program into LiteSVM and checks the complete flow:

1. Initialize a vault.
2. Deposit `500_000_000` lamports.
3. Withdraw `100_000_000` lamports.
4. Close the vault.
5. Verify that the state account and vault balance are removed.

Run the Rust tests with:

```bash
cargo test
```

Or use the Anchor script configured in `Anchor.toml`:

```bash
anchor test
```

## Project structure

```text
.
├── Anchor.toml
├── Cargo.toml
└── programs/
    └── q3_26_vault/
        ├── src/
        │   ├── instructions/
        │   │   ├── close.rs
        │   │   ├── deposit.rs
        │   │   ├── initialize.rs
        │   │   └── withdraw.rs
        │   ├── constants.rs
        │   ├── error.rs
        │   ├── lib.rs
        │   └── state.rs
        └── tests/
            └── test_initialize.rs
```

## Error conditions

- `InvalidAmount`: the deposit or withdrawal amount is zero.
- `InsufficientFunds`: the requested withdrawal is greater than the lamports available above the vault's rent reserve.

## License

No license has been specified for this project yet.

<img width="735" height="306" alt="Vault tests" src="https://github.com/user-attachments/assets/a7068e50-ff61-4ed4-9fed-84e46f85dc55" />

