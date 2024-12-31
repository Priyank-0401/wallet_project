# Token Wallet Project Documentation

## Description

This project is a token wallet built on the Internet Computer (ICP) platform. It enables token management functionalities such as balance initialization, querying balances, and token transfers between users.

## Features

- **Balance Initialization**: Initializes user balances with a default value of 1000 tokens.
- **Balance Retrieval**: Allows users to query their current token balance.
- **Token Transfer**: Enables users to transfer tokens to others.

## Setup and Installation

### Prerequisites

Ensure that the following software is installed on your system:

- [dfx](https://internetcomputer.org/docs/current/developers-guide/install/dfx/) for managing canisters on the ICP network.
- [Rust](https://www.rust-lang.org/learn/get-started) to develop and compile the backend logic.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) for managing Rust packages.

### Installing Dependencies

1. In the project directory, run the following command to start the local ICP testnet:

    ```bash
    dfx start --background
    ```

2. Deploy the backend canister to the testnet:

    ```bash
    dfx deploy
    ```

This will set up the local testnet and deploy the backend canister.

## Deployment Instructions

### Starting the Local ICP Testnet

To start a local ICP testnet in the background, run the following command:

```bash
dfx start --background
```

# Wallet Project Backend

## Deployment Instructions

### Deploy the Canisters

To deploy the backend canister to the local testnet, run:

```bash
dfx deploy
```

### Get the Canister ID

After deploying, retrieve the canister ID by running:

```bash
dfx canister id wallet_project_backend
```

## Testing Instructions

To ensure that the canister's logic functions correctly, you can run the unit tests included in the project.

### Running Unit Tests

To execute the tests, run:

```bash
cargo test
```

## Unit Test Coverage

The project includes tests for the following key areas:

- **Balance Initialization**: Verifies that the balance is set to 1000 tokens upon initialization.
- **Balance Retrieval**: Ensures that the balance retrieval mechanism works correctly.
- **Token Transfer**: Verifies that tokens can be transferred from one user to another, including handling insufficient balance scenarios.
