## ICSQLite
ICSQLite is a cloud SQLite database on Internet Computer and provides SDK for developers to use.  
Our goal is to help developers quickly migrate web2 applications to Internet Computer. 


## Usage

In your Cargo.toml:

```toml
[dependencies]
ic-sqlite = "0.1.0"
```

## Limitations & Suggestions
Limited by the total number of cycles of a call, if the number of rows retrieved by a single SQL query exceeds a certain amount, the call will crash.

#### SQL statement suggestions
* Strictly follow the rules of database optimization
* Index building must be an empty table
* Where query must be filtered for primary key or index field
* Less use NOT,!=,<>,!<,!> NOT EXISTS, NOT IN, NOT LIKE, OR, they will ignore the index and cause a full table scan

#### [Performance benchmarks for SQL commands](https://github.com/froghub-io/ic-sqlite/tree/main/examples/bench)
| SQL <br/> commands               | performance counter <br/> 1w single table data | performance counter <br/> 10w single table data | performance counter <br/> 50w single table data | performance counter <br/> 100w single table data |
|----------------------------------|------------------------------------------------|-------------------------------------------------|-------------------------------------------------|--------------------------------------------------|
| create table                     | 1194347                                        | 1433766                                         | 2565609                                         | 4066020                                          | 
| create index <br/> (empty table) | 884588                                         | 1122419                                         | 2241730                                         | 3601724                                          |
| count                            | 209847                                         | 2995943                                         | 15183853                                        | 30392494                                         | 
 | insert                           | 350256                                         | 349635                                          | 351731                                          | 355381                                           | 
| select <br/> (where primary key) | 265363                                         | 265960                                          | 265345                                          | 268112                                           | 
| select <br/> (where index field) | 312389                                         | 314594                                          | 314666                                          | 319276                                           | 
| select <br/> (where like field)  | 178263088                                      | 1784671532                                      | limit for single message execution              | limit for single message execution               | 
| update <br/> (where primary key) | 385492                                         | 389192                                          | 391599                                          | 394111                                           | 
| update <br/> (where index filed) | 239384                                         | 237908                                          | 237993                                          | 240998                                           | 
| delete <br/> (where primary key) | 429190                                         | 259541                                          | 419615                                          | 423064                                           |

## [IC Canister Simple example usage](https://github.com/froghub-io/ic-sqlite/tree/main/examples/backend)

## Data migration suggestions & Debugging
* Provide an interface for executing sql statements with super-management authority
* Export standard sql statements for offline data and upload in batches
* Debugging Get the data by running the sql statement through the super-management interface