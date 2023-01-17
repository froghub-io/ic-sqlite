#!/bin/bash

# create table person
dfx canister call backend execute 'create table person ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER )'

# create person name index
dfx canister call backend execute 'create index name on person(name)'

TOTAL=1000000
COUNTER=0
PER=10000
while [ $COUNTER -lt $TOTAL ];
do

  CUR=`expr $COUNTER + $PER`
  echo "--- ${CUR} ---"

  dfx canister call backend bench1_insert_person "(${COUNTER}, $PER)"

  COUNTER=`expr $COUNTER + $PER`

  # create table person
  dfx canister call backend execute "create table person$COUNTER ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER )"

  # create person name index
  dfx canister call backend execute "create index name$COUNTER on person$COUNTER(name)"

  # count performance counter
  dfx canister call backend count '("person")'

  # insert performance counter
  dfx canister call backend bench1_insert_person_one "(${COUNTER})"

  # query_by_id performance counter
  dfx canister call backend bench1_query_person_by_id "(${COUNTER})"

  # query_by_name performance counter
  dfx canister call backend bench1_query_person_by_name "(${COUNTER})"

  # query_by_like_name performance counter
  dfx canister call backend bench1_query_person_by_like_name "(${COUNTER})"

  # bench1_query_person_by_limit_offset performance counter
  HALF_COUNTER=`expr $COUNTER / 2`
  dfx canister call backend bench1_query_person_by_limit_offset "(10, ${HALF_COUNTER})"

  # update_by_name performance counter
  dfx canister call backend bench1_update_person_by_name "(${COUNTER})"

  # update_by_id performance counter
  dfx canister call backend bench1_update_person_by_id "(${COUNTER})"

  # delete_by_id performance counter
  dfx canister call backend bench1_delete_person_by_id "(${COUNTER})"

done