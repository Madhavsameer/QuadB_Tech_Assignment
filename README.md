
# ICP Token Wallet

This project is a simple Rust-based token wallet for the Internet Computer Protocol (ICP) blockchain. It demonstrates how to manage token transactions using a basic wallet model, which includes functionalities for sending and receiving tokens, checking balance, and performing unit tests to verify wallet operations.

## Table of Contents

- [Project Structure](#project-structure)
- [Requirements](#requirements)
- [Installation and Setup](#installation-and-setup)
- [Running the Code](#running-the-code)
- [Testing the Code](#testing-the-code)
- [Usage](#usage)

## Project Structure

The code is organized as follows:

```
icp_token_wallet/
├── src/
│   ├── main.rs               # Main entry point (optional for CLI)
│   ├── lib.rs                # Makes the wallet functionality accessible as a library
│   └── token_wallet.rs       # Core wallet logic and token functionalities
└── tests/
    └── token_wallet_test.rs  # Contains unit tests for wallet functions
```

### Files

- **src/token_wallet.rs**: Implements the core wallet functionality, including methods to send tokens, receive tokens, and check balance.
- **src/lib.rs**: Allows `token_wallet.rs` to be accessed as a library, enabling modular imports in test files.
- **tests/token_wallet_test.rs**: Contains tests for each wallet function to ensure that token transactions behave as expected.

## Requirements

To run this project, ensure you have:

- **Rust** installed (version 1.56 or later).
- A **Rust-compatible IDE or environment** (e.g., Project IDX, Visual Studio Code).

## Installation and Setup

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd icp_token_wallet
   ```

2. Update dependencies in `Cargo.toml`:

   ```toml
   [package]
   name = "icp_token_wallet"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   candid = "0.7.8"
   ic-cdk = "0.6.2"

   [lib]
   name = "icp_token_wallet"
   path = "src/lib.rs"
   ```

## Running the Code

To build and run the application:

```bash
cargo build
cargo run
```

### Example Usage

To interact with the wallet's basic functionality, you can use the methods defined in `token_wallet.rs` such as `send_tokens`, `receive_tokens`, and `get_balance`.

## Testing the Code

This project includes unit tests in `tests/token_wallet_test.rs`. To run the tests, execute:

```bash
cargo test
```

Expected output for tests:

- **test_send_tokens**: Verifies that tokens can be sent from one wallet to another.
- **test_receive_tokens**: Ensures a wallet can correctly receive tokens and reflect the updated balance.
- **test_insufficient_funds**: Tests that sending more tokens than available in the balance results in an error.

## Usage

The main functions available in `Wallet` are:

1. **send_tokens(amount: u64, recipient: &Wallet) -> Result<(), String>**: Transfers tokens from one wallet to another if sufficient balance exists.

2. **receive_tokens(amount: u64)**: Adds tokens to a wallet's balance.

3. **get_balance() -> u64**: Returns the current balance of the wallet.

---
