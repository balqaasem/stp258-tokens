# Setheum Tokenization Protocol 258 - SERP
Multi-Currency Stablecoin SERP Module

## Overview

The stp258 module provides fungible multiple stable currencies functionality that implements `SettCurrency` traits, the `SerpTes` trait and the `SerpMarket` trait that enables the Serping up and down of the currencies supply through Setheum's serping technology for currency supply elasticity.

The stp258 module provides functions for:

- Querying and setting the balance of a given account.
- Getting and managing total issuance.
- Balance transfer between accounts.
- Depositing and withdrawing balance.
- Slashing an account balance.
- Minting and Burning currencies.
- Expanding and Contracting Supply of Currencies by Serping
## Acknowledgement & Reference

This Pallet is inspired by the [ORML Tokens](https://github.com/open-web3-stack/open-runtime-module-library/blob/master/tokens) Pallet developed by [Open Web3 Stack](https://github.com/open-web3-stack/), for reference check [The ORML Repo](https://github.com/open-web3-stack/open-runtime-module-library).
 
## Test & Build

Run `cargo build` to build.
Run `cargo test` to test.

    build:

    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v2
    - name: Install toolchain
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: nightly-2021-02-17
        target: wasm32-unknown-unknown
        default: true
    - name: Install Wasm toolchain
      run: rustup target add wasm32-unknown-unknown
    - name: Install clippy
      run: rustup component add clippy
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
