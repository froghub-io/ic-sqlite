#!/bin/bash

# create table person
dfx canister call backend execute 'create table person ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER )'

# create person name index
dfx canister call backend execute 'create index name on person(name)'

# insert person
dfx canister call backend execute 'insert into person (name, age, gender) values ("a", 15, 0);'
dfx canister call backend execute 'insert into person (name, age, gender) values ("b", 16, 1);'
dfx canister call backend execute 'insert into person (name, age, gender) values ("c", 17, 0);'

# query person
dfx canister call backend query 'select * from person limit 10'