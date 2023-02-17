## ICSQLite
ICSQLite is a cloud SQLite database on Internet Computer and provides SDK for developers to use.  
Our goal is to help developers quickly migrate web2 applications to Internet Computer. 


## Usage

In your Cargo.toml:

```toml
[dependencies]
ic-sqlite = { version = "0.1.0", git = "https://github.com/froghub-io/ic-sqlite.git", branch = "hashmap_memory" }
```

## Limitations
Limited by the total number of cycles of a call, if the number of rows retrieved by a single SQL query exceeds a certain amount, the call will crash. The following is the test results, and I hope you can provide more feedback

![report.png](https://github.com/froghub-io/ic-sqlite/blob/main/examples/static/report.png)


## [IC Canister Simple example usage](https://github.com/froghub-io/ic-sqlite/tree/main/examples/backend)