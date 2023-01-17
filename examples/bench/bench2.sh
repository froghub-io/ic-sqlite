#!/bin/bash

# create table person
dfx canister call backend execute 'create table person2 ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER, data TEXT NOT NULL )'

# create person name index
dfx canister call backend execute 'create index name2 on person2(name)'

TOTAL=1000000
COUNTER=0
PER=10000
while [ $COUNTER -lt $TOTAL ];
do

  CUR=`expr $COUNTER + $PER`
  echo "--- ${CUR} ---"

  dfx canister call backend bench2_insert_person2 "(${COUNTER}, $PER)"

  COUNTER=`expr $COUNTER + $PER`

  # create table person2
  dfx canister call backend execute "create table person2$COUNTER ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER, data BLOB )"

  # create person2 name index
  dfx canister call backend execute "create index name2$COUNTER on person2$COUNTER(name)"

  # count performance counter
  dfx canister call backend count '("person2")'

  # insert performance counter
  dfx canister call backend bench2_insert_person2_one "(${COUNTER})"

  # query2_by_id performance counter
  dfx canister call backend bench2_query_person2_by_id "(${COUNTER})"

  # query2_by_name performance counter
  dfx canister call backend bench2_query_person2_by_name "(${COUNTER})"

  # query2_by_like_name performance counter
  dfx canister call backend bench2_query_person2_by_like_name "(${COUNTER})"

  # bench2_query_person2_by_limit_offset performance counter
  HALF_COUNTER=`expr $COUNTER / 2`
  dfx canister call backend bench2_query_person2_by_limit_offset "(10, ${HALF_COUNTER})"

  # update_by_name performance counter
  dfx canister call backend bench2_update_person2_by_name "(${COUNTER})"

  # update_by_id performance counter
  dfx canister call backend bench2_update_person2_by_id "(${COUNTER})"

  # delete_by_id performance counter
  dfx canister call backend bench2_delete_person2_by_id "(${COUNTER})"

done