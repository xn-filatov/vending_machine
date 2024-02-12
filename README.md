# Vending Machine

This is a Vemfing Machine cosmwasm smart contract solution built with Rust.

## Queries:

- ItemsCount -> returns numbers of items of each kind in the machine

## Functions
- GetItem (available to anyone) -> requires type of item and reduces the number of items of that type by 1
- Refill (for owner only) -> requires the amounts of refill -> increases the number if items by amount

## Prerequisites
Before running the solution make sure you have installed `Rust v1.76.0`


### Building and testing
2. Run `cargo test` to run the unit tests.
3. Run `cargo wasm` or `cargo wasm-debug` to build the app.
4. Run `cargo schema` to the contract's schema.

### Deployment
To deploy the contract you can follow the link: `https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment`
