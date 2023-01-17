## Bench Test Examples Container

## Summary
Bench Test Example of using ICSQLite in IC Canister

## Setup

To build and install this code, you will need:

- Git
- [DFX] version 0.9.0
- [Rust] version 1.55.0 or later

```sh
git clone https://github.com/froghub-io/ic-sqlite.git
cd examples/bench 
```

To start the local replica before installing the canister:

```sh
dfx start --background --clean
```

Register, build and deploy the project.
```sh
dfx deploy
```

Run bench scripts
```sh
bash bench1.sh
bash bench2.sh
```