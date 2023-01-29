#[macro_use]
extern crate ic_cdk_macros;
#[macro_use]
extern crate serde;

use ic_cdk::api::call::RejectionCode;
use candid::CandidType;
use rusqlite::types::Type;

#[query]
fn balance() -> u64 {
    ic_cdk::api::canister_balance()
}

#[query]
fn instruction_counter() -> u64 {
    ic_cdk::api::instruction_counter()
}

#[update]
fn execute(sql: String) -> Result {
    let conn = ic_sqlite::CONN.lock().unwrap();
    return match conn.execute(
        &sql,
        []
    ) {
        Ok(e) => Ok(format!("{:?}", e)),
        Err(err) => Err(Error::CanisterError {message: format!("{:?}", err) })
    }
}

#[query]
fn query(sql: String) -> QueryResult {
    let conn = ic_sqlite::CONN.lock().unwrap();
    let mut stmt = conn.prepare(&sql).unwrap();
    let cnt = stmt.column_count();
    let mut rows = stmt.query([]).unwrap();
    let mut res: Vec<Vec<String>> = Vec::new();
    loop {
        match rows.next() {
            Ok(row) => {
                match row {
                    Some(row) => {
                        let mut vec: Vec<String> = Vec::new();
                        for idx in 0..cnt {
                            let v = row.get_ref_unwrap(idx);
                            match v.data_type() {
                                Type::Null => {  vec.push(String::from("")) }
                                Type::Integer => { vec.push(v.as_i64().unwrap().to_string()) }
                                Type::Real => { vec.push(v.as_f64().unwrap().to_string()) }
                                Type::Text => { vec.push(v.as_str().unwrap().parse().unwrap()) }
                                Type::Blob => { vec.push(hex::encode(v.as_blob().unwrap())) }
                            }
                        }
                        res.push(vec)
                    },
                    None => break
                }
            },
            Err(err) => return Err(Error::CanisterError {message: format!("{:?}", err) })
        }
    }
    Ok(res)
}

#[derive(CandidType, Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type Result<T = String, E = Error> = std::result::Result<T, E>;

type QueryResult<T = Vec<Vec<String>>, E = Error> = std::result::Result<T, E>;

impl From<(RejectionCode, String)> for Error {
    fn from((code, message): (RejectionCode, String)) -> Self {
        match code {
            RejectionCode::CanisterError => Self::CanisterError { message },
            _ => Self::InvalidCanister,
        }
    }
}

