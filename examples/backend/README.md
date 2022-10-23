## Examples Container

## Summary
Example of using ICSQLite in IC Canister

## Setup

To build and install this code, you will need:

- Git
- [DFX] version 0.9.0
- [Rust] version 1.55.0 or later

```sh
git clone https://github.com/froghub-io/ic-sqlite.git
cd examples/backend 
```

To start the local replica before installing the canister:

```sh
dfx start --background --clean
```

Register, build and deploy the project.
```sh
dfx deploy
```

![deploy.png](https://github.com/froghub-io/ic-sqlite/blob/main/examples/static/deploy.png)

Access the backend address

![actions.png](https://github.com/froghub-io/ic-sqlite/blob/main/examples/static/actions.png)