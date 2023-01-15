#[macro_use]
extern crate ic_cdk_macros;
#[macro_use]
extern crate serde;

use ic_cdk::api::call::RejectionCode;
use candid::CandidType;

#[update]
fn create() -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        "create table person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        []
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[query]
fn query(params: QueryParams) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from person limit ?1 offset ?2") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let person_iter = match stmt.query_map((params.limit, params.offset), |row| {
        Ok(PersonQuery {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut persons = Vec::new();
    for person in person_iter {
        persons.push(person.unwrap());
    }
    let res = serde_json::to_string(&persons).unwrap();
    Ok(res)
}

#[query]
fn query_filter(params: FilterParams) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = match conn.prepare("select * from person where name=?1") {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let person_iter = match stmt.query_map((params.name, ), |row| {
        Ok(PersonQuery {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2).unwrap(),
        })
    }) {
        Ok(e) => e,
        Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
    };
    let mut persons = Vec::new();
    for person in person_iter {
        persons.push(person.unwrap());
    }
    let res = serde_json::to_string(&persons).unwrap();
    Ok(res)
}

#[update]
fn insert(person: Person) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        "insert into person (name, age) values (?1, ?2);",
        (person.name, person.age,)
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn delete(id: usize) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        "delete from person where id=?1",
        (id,)
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[update]
fn update(params: UpdateParams) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        "update person set name=?1 where id=?2",
        (params.name, params.id)
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct Person {
    name: String,
    age: usize,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct PersonQuery {
    id: usize,
    name: String,
    age: usize,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct QueryParams {
    limit: usize,
    offset: usize,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct FilterParams {
    name: String,
}

#[derive(CandidType, Debug, Serialize, Deserialize, Default)]
struct UpdateParams {
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