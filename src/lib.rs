mod vfs;

use std::collections::HashMap;
use std::sync::{RwLock, Mutex, Arc};
use once_cell::sync::Lazy;
use lazy_static::lazy_static;
use rusqlite::{Connection, OpenFlags};
use sqlite_vfs::register;

static STORAGE: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

lazy_static! {
    pub static ref CONN: Arc<Mutex<Connection>> = {
        register("vfs", vfs::PagesVfs::<4096>::default(), true).unwrap();
        let conn = Connection::open_with_flags_and_vfs(
            "main.db",
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            "vfs",
        ).unwrap();
        conn.execute_batch(
            r#"
            PRAGMA page_size=4096;
            PRAGMA journal_mode=MEMORY;
            "#,
        ).unwrap();

        return Arc::new(Mutex::new(conn));
    };

}

pub fn get_storage() -> HashMap<String, String> {
    return std::mem::take(&mut *STORAGE.write().unwrap());
}

pub fn set_storage(data: HashMap<String, String>) {
    *STORAGE.write().unwrap() = data;
}

fn page_count() -> u32 {
    return match STORAGE.read().unwrap().get("page_count") {
        Some(v) => v.parse::<u32>().expect("page count get fail"),
        None => 0
    }
}

fn set_page_count(pc: u32) {
    STORAGE.write().unwrap().insert("page_count".to_string(), pc.to_string());
}

fn get_page(ix: u32, ptr: &mut [u8]) {
    match STORAGE.read().unwrap().get(&ix.to_string()) {
        Some(v) => {
            let buf:Vec<u8> = hex::decode(v).unwrap();
            ptr.copy_from_slice(&buf);
        },
        None => {}
    }
}

fn put_page(ix: u32, ptr: String) {
    STORAGE.write().unwrap().insert(ix.to_string(), ptr);
    if ix + 1 >= page_count() {
        set_page_count(ix + 1);
    }
}

fn del_page(ix: u32) {
    if STORAGE.read().unwrap().contains_key(&ix.to_string()) {
        STORAGE.write().unwrap().remove(&ix.to_string());
        if ix + 1 >= page_count() {
            set_page_count(ix);
        }
    }
}

fn conn_sleep(ms: u32) {
    std::thread::sleep(std::time::Duration::from_secs(ms.into()));
}

fn put(ix: String, ptr: String) {
    STORAGE.write().unwrap().insert(ix, ptr);
}

fn get(ix: String) -> Option<Vec<u8>> {
    match STORAGE.read().unwrap().get(&ix) {
        Some(v) => Some(hex::decode(v).unwrap()),
        None => None
    }
}

fn del(ix: String) {
    if STORAGE.read().unwrap().contains_key(&ix) {
        STORAGE.write().unwrap().remove(&ix);
    }
}

fn exists(ix: String) -> bool {
    STORAGE.read().unwrap().contains_key(&ix)
}

fn set_len(ix: String, size: u64) {
    let data = get(ix.clone());
    let mut buf = vec![0u8; size as usize];
    if let Some(mut data) = data {
        if size as usize > data.len() {
            let len = data.len();
            (0..size as usize - len).for_each(|_|{
                data.push(0u8);
            });
            buf = data;
        } else {
            buf.copy_from_slice(&data[0..size as usize]);
        }
    }
    put(ix.clone(), hex::encode(buf));
}