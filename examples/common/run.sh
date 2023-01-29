#!/bin/bash

echo "--- create table person"
dfx canister call backend execute 'create table person ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER )'

echo "--- create person name index"
dfx canister call backend execute 'create index name on person(name)'

echo "--- insert person"
dfx canister call backend execute 'insert into person (name, age, gender) values ("a", 15, 0);'
dfx canister call backend execute 'insert into person (name, age, gender) values ("b", 16, 1);'
dfx canister call backend execute 'insert into person (name, age, gender) values ("c", 17, 0);'

echo "--- query person"
dfx canister call backend query 'select * from person limit 10'

echo "--- delete person with name=a"
dfx canister call backend execute 'delete from person where name="a"'

echo "--- update person with name=b"
dfx canister call backend execute 'update person set name="abc" where name="b"'

echo "--- query person"
dfx canister call backend query 'select * from person limit 10'