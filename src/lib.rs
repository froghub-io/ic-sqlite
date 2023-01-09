mod vfs;

use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;
use rusqlite::{Connection, OpenFlags};
use sqlite_vfs::register;
use ic_cdk::api::stable::{stable64_size, stable64_grow, StableMemoryError};

lazy_static! {
    pub static ref CONN: Arc<Mutex<Connection>> = {
        register("vfs", vfs::PagesVfs::default(), true).unwrap();
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

const WASM_PAGE_SIZE_IN_BYTES: u64 = 64 * 1024; // 64KB

/// Gets capacity of the stable memory in bytes.
pub fn stable_capacity() -> u64 {
    stable64_size() << 16
}

/// Attempts to grow the memory by adding new pages.
pub fn stable_grow_bytes(size: u64) -> Result<u64, StableMemoryError> {
    let added_pages = (size as f64 / WASM_PAGE_SIZE_IN_BYTES as f64).ceil() as u64;
    stable64_grow(added_pages)
}