#[macro_use]
extern crate ic_cdk_macros;
#[macro_use]
extern crate serde;

use ic_cdk::api::call::RejectionCode;
use candid::CandidType;

#[update]
fn execute(sql: String) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        &sql,
        []
    ) {
        Ok(_) => Ok(format!("execute performance_counter: {:?}", ic_cdk::api::performance_counter(0))),
        Err(err) => Err(Error::CanisterError {message: format!("execute: {:?}", err) })
    }
}

#[query]
fn count(table_name: String) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare(&format!("select count(*) from {:?}", table_name)) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut iter = match stmt.query_map([], |row| {
        let count: u64 = row.get(0).unwrap();
        Ok(count)
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("count: {:?}", err) })
    };
    let count = iter.next().unwrap().unwrap();
    ic_cdk::eprintln!("count: {:?}", count);
    Ok(format!("count performance_counter: {:?}", ic_cdk::api::performance_counter(0)))
}

#[update]
fn bench1_insert_person(offset: usize, count: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    for i in 0..count {
        let id = offset + i + 1;
        match conn.execute(
            "insert into person (name, age, gender) values (?1, ?2, ?3);",
            (format!("person{:?}", id), 18 + id % 10, id % 2)
        ) {
            Ok(_) => {},
            Err(err) =>  return Err(Error::CanisterError {message: format!("bench1_insert_person: {:?}", err) })
        }
    }
    Ok(String::from("bench1_insert_person OK"))
}

#[update]
fn bench1_insert_person_one(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let id = offset + 1;
    match conn.execute(
        "insert into person (name, age, gender) values (?1, ?2, ?3);",
        (format!("person{:?}", id), 18 + id % 10, id % 2)
    ) {
        Ok(_) => Ok(format!("insert performance_counter: {:?}", ic_cdk::api::performance_counter(0))),
        Err(err) => Err(Error::CanisterError {message: format!("insert: {:?}", err) })
    }
}

#[query]
fn bench1_query_person_by_id(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let id = offset + 1;
    let mut stmt = match conn.prepare("select * from person where id=?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_id: {:?}", err) })
    };
    let iter = match stmt.query_map((id,), |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap()
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_id: {:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    ic_cdk::eprintln!("query_by_id: {:?}", res);
    Ok(format!("query_by_id performance_counter: {:?}", ic_cdk::api::performance_counter(0)))
}

#[query]
fn bench1_query_person_by_name(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let name = format!("person{:?}", offset + 1);
    let mut stmt = match conn.prepare("select * from person where name=?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_name: {:?}", err) })
    };
    let iter = match stmt.query_map((name,), |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap()
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_name: {:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    ic_cdk::eprintln!("query_by_name: {:?}", res);
    Ok(format!("query_by_name performance_counter: {:?}", ic_cdk::api::performance_counter(0)))
}

#[query]
fn bench1_query_person_by_like_name(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let name = format!("person{:?}", offset + 1);
    let mut stmt = match conn.prepare("select * from person where name like ?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let iter = match stmt.query_map((format!("{:?}%", name),), |row| {
        Ok(Person {
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
    ic_cdk::eprintln!("query_by_like_name: {:?}", res);
    Ok(format!("query_by_like_name performance_counter: {:?}", ic_cdk::api::performance_counter(0)))
}

#[query]
fn bench1_query_person_by_limit_offset(limit: usize, offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from person limit ?1 offset ?2") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_limit_offset: {:?}", err) })
    };
    let iter = match stmt.query_map((limit, offset), |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
            gender: row.get(3).unwrap()
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("query_by_limit_offset: {:?}", err) })
    };
    let mut arr = Vec::new();
    for ite in iter {
        arr.push(ite.unwrap());
    }
    let res = serde_json::to_string(&arr).unwrap();
    ic_cdk::eprintln!("query_by_limit_offset: {:?}", res);
    Ok(format!("query_by_limit_offset performance_counter: {:?}", ic_cdk::api::performance_counter(0)))
}

#[update]
fn bench1_update_person_by_id(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let id = offset + 1;
    return match conn.execute(
        "update person set name=?1 where id=?2",
        (String::from("person_id"), id)
    ) {
        Ok(_) => Ok(format!("update_by_id performance_counter: {:?}", ic_cdk::api::performance_counter(0))),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn bench1_update_person_by_name(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let name = format!("{:?}", offset + 1);
    return match conn.execute(
        "update person set name=?1 where name=?2",
        (String::from("person_name"), name)
    ) {
        Ok(_) => Ok(format!("update_by_name performance_counter: {:?}", ic_cdk::api::performance_counter(0))),
        Err(err) => Err(Error::CanisterError {message: format!("update_by_name: {:?}", err) })
    }
}

#[update]
fn bench1_delete_person_by_id(offset: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let id = offset + 1;
    return match conn.execute(
        "delete from person where id=?1",
        (id,)
    ) {
        Ok(_) => Ok(format!("delete performance_counter: {:?}", ic_cdk::api::performance_counter(0))),
        Err(err) => Err(Error::CanisterError {message: format!("delete: {:?}", err) })
    }
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Person {
    id: u64,
    name: String,
    age: u32,
    gender: u8
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