#[macro_use]
extern crate ic_cdk_macros;
#[macro_use]
extern crate serde;

use ic_cdk::api::call::RejectionCode;
use candid::CandidType;

#[query]
fn balance() -> u64 {
    ic_cdk::api::canister_balance()
}

#[query]
fn instruction_counter() -> u64 {
    ic_cdk::api::instruction_counter()
}

#[query]
fn performance_counter() -> u64 {
    ic_cdk::api::performance_counter(0)
}

#[update]
fn test() -> u64 {
    ic_cdk::eprintln!("test performance_counter: {:?}", ic_cdk::api::performance_counter(0));
    let size = 0;
    ic_cdk::eprintln!("test performance_counter: {:?}", ic_cdk::api::performance_counter(0));
    size
}

#[update]
fn execute(sql: String) -> Result {
    ic_cdk::eprintln!("execute performance_counter: {:?}", ic_cdk::api::performance_counter(0));
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        &sql,
        []
    ) {
        Ok(e) => {
            ic_cdk::eprintln!("execute performance_counter: {:?}", ic_cdk::api::performance_counter(0));
            Ok(format!("{:?}", e))
        },
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn insert_course(name: String, credit: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    match conn.execute(
        "insert into course (name, credit) values (?1, ?2);",
        (name.clone(), credit)
    ) {
        Ok(e) => Ok(format!("{:?}: {:?}", name, e)),
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn insert_subject(offset: usize, count: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    for i in 0..count {
        let id = offset + i + 1;
        match conn.execute(
            "insert into subject (name) values (?1);",
            [format!("sub{:?}", id)]
        ) {
            Ok(_) => {},
            Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
        }
    }
    Ok(format!("{:?}", offset + count))
}

#[update]
fn insert_student(offset: usize, count: usize, subject_count: usize, course_count: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    for i in 0..count {
        let id = offset + i + 1;
        match conn.execute(
            "insert into student (name, age, gender) values (?1, ?2, ?3);",
            (format!("stu{:?}", id), 10 + i % 8, i % 2)
        ) {
            Ok(_) => {},
            Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
        }

        match conn.execute(
            "insert into have (subject_id, student_id) values (?1, ?2);",
            (1 + id % subject_count, id)
        ) {
            Ok(_) => {},
            Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
        }

        for j in 0..(id % course_count) {
            match conn.execute(
                "insert into study (student_id, course_id, source) values (?1, ?2, ?3);",
                (id, j + 1, 20 + id % 80)
            ) {
                Ok(_) => {},
                Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
            }
        }
    }
    Ok(format!("{:?}", offset + count))
}

#[query]
fn count(table: CountTable) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare(&format!("select count(*) from {:?}", table.table_name)) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut iter = match stmt.query_map([], |row| {
        let count: u64 = row.get(0).unwrap();
        Ok(count)
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let count = iter.next().unwrap().unwrap();
    Ok(format!("{:?}", count))
}

#[query]
fn query_course() -> Result {
    ic_cdk::eprintln!("query_course performance_counter: {:?}", ic_cdk::api::performance_counter(0));
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from course") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let iter = match stmt.query_map([], |row| {
        Ok(Course {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            credit: row.get(2).unwrap(),
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    ic_cdk::eprintln!("query_course performance_counter: {:?}", ic_cdk::api::performance_counter(0));
    Ok(res)
}

#[query]
fn query_subject(filter: Filter) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from subject limit ?1 offset ?2") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let iter = match stmt.query_map((filter.limit, filter.offset), |row| {
        Ok(Subject {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    Ok(res)
}

#[query]
fn query_student(filter: Filter) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from student limit ?1 offset ?2") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let iter = match stmt.query_map((filter.limit, filter.offset), |row| {
        Ok(Student {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap()
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    Ok(res)
}

#[query]
fn query_student_by_id(id: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from student where id=?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let iter = match stmt.query_map((id,), |row| {
        Ok(Student {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap()
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    Ok(res)
}

#[update]
fn update_student_name_by_id(id: usize, name: String) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        "update student set name=?1 where id=?2",
        (name, id)
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn delete(params: TableAndId) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        &format!("delete from {:?} where id=?1", params.table_name),
        (params.id,)
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[query]
fn statistics_student_number_in_subject(id: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select count(*) from have where subject_id=?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut iter = match stmt.query_map((id, ), |row| {
        let count: u64 = row.get(0).unwrap();
        Ok(count)
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let count = iter.next().unwrap().unwrap();
    Ok(format!("{:?}", count))
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Course {
    id: u64,
    name: String,
    credit: u32,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Subject {
    id: u64,
    name: String,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Student {
    id: u64,
    name: String,
    age: u32,
    gender: u8
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Filter {
    limit: usize,
    offset: usize,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct CountTable {
    table_name: String,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct TableAndId {
    table_name: String,
    id: usize
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Update {
    id: usize,
    name: String
}

#[derive(CandidType, Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type Result<T = String, E = Error> = std::result::Result<T, E>;

impl From<(RejectionCode, String)> for Error {
    fn from((code, message): (RejectionCode, String)) -> Self {
        match code {
            RejectionCode::CanisterError => Self::CanisterError { message },
            _ => Self::InvalidCanister,
        }
    }
}