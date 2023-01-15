# create student table
dfx canister call backend execute 'create table student ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, gender INTEGER )'
# create teacher table
dfx canister call backend execute 'create table subject ( id INTEGER PRIMARY KEY, name TEXT NOT NULL )'
# create course table
dfx canister call backend execute 'create table course ( id INTEGER PRIMARY KEY, name TEXT NOT NULL, credit INTEGER )'
# create study table
dfx canister call backend execute 'create table study ( id INTEGER PRIMARY KEY, student_id INTEGER, course_id INTEGER, source INTEGER )'
# create have table
dfx canister call backend execute 'create table have ( id INTEGER PRIMARY KEY, subject_id INTEGER, student_id INTEGER )'

# create study student_id index
dfx canister call backend execute 'create index student_id on study(student_id)'

# create have subject_id index
dfx canister call backend execute 'create index subject_id on have(subject_id)'

# insert course
COURSE=9
dfx canister call backend insert_course '("Chinese", 5)'
dfx canister call backend insert_course '("Math", 5)'
dfx canister call backend insert_course '("English", 5)'
dfx canister call backend insert_course '("Physical", 2)'
dfx canister call backend insert_course '("Biology", 3)'
dfx canister call backend insert_course '("Chemistry", 3)'
dfx canister call backend insert_course '("Thought", 3)'
dfx canister call backend insert_course '("Computer", 3)'
dfx canister call backend insert_course '("Anatomy", 3)'

# insert subject
SUBJECT=1000000
COUNTER=0
while [ $COUNTER -lt $SUBJECT ];
do
  dfx canister call backend insert_subject "(${COUNTER}, 10000)"
  COUNTER=`expr $COUNTER + 10000`
done

# insert student
STUDENT=100000
COUNTER=0
while [ $COUNTER -lt $STUDENT ];
do
  dfx canister call backend insert_student "(${COUNTER}, 2000, ${SUBJECT}, ${COURSE})"
  COUNTER=`expr $COUNTER + 2000`
done





