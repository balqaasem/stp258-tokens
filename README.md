# Setheum Tokenization Protocol 258
Multi-Currency Stablecoin SERP Module

## Overview

The stp258 module provides fungible multiple stable currencies functionality that implements `SettCurrency` trait.

The stp258 module provides functions for:

- Querying and setting the balance of a given account.
- Getting and managing total issuance.
- Balance transfer between accounts.
- Depositing and withdrawing balance.
- Slashing an account balance.
- Minting and Burning currencies.
- Fetching prices for currencies.
- A basket_token could be made by combining a basket of prices into one in any desired ratio. Could be done on runtime, the basket_token price_of_pegs and basket_ratio logic could be defined in an offchain worker and fed on-chain.
 
 It also implement an atomic swap, to atomically swap currencies 
  
 - `create_swap` - called by a sender to register a new atomic swap
 - `claim_swap` - called by the target to approve a swap
 - `cancel_swap` - may be called by a sender after a specified duration.

## Acknowledgement & Reference

This Pallet is inspired by the [ORML Tokens](https://github.com/open-web3-stack/open-runtime-module-library/blob/master/tokens) Pallet developed by [Open Web3 Stack](https://github.com/open-web3-stack/), for reference check [The ORML Repo](https://github.com/open-web3-stack/open-runtime-module-library).

This Pallet is inspired by the [Stablecoin](https://github.com/apopiak/stablecoin) Pallet developed by [Alexander Popiak](https://github.com/apopiak), for reference check [The Apopiak/Stablecoin Repo](https://github.com/apopiak/stablecoin).
