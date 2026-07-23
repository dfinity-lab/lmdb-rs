//! Idiomatic and safe APIs for interacting with the
//! [Lightning Memory-mapped Database (LMDB)](https://symas.com/lmdb).

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/lmdb-rkv/0.14.0")]

extern crate byteorder;
extern crate libc;
extern crate lmdb_sys as ffi;

#[cfg(test)]
extern crate tempfile;
#[macro_use]
extern crate bitflags;

pub use cursor::{Cursor, Iter, IterDup, RoCursor, RwCursor};
pub use crate::database::Database;
pub use environment::{Environment, EnvironmentBuilder, Info, Stat};
pub use crate::error::{Error, Result};
pub use crate::flags::*;
pub use crate::transaction::{InactiveTransaction, RoTransaction, RwTransaction, Transaction};

macro_rules! lmdb_try {
    ($expr:expr) => {{
        match $expr {
            ::ffi::MDB_SUCCESS => (),
            err_code => return Err(crate::Error::from_err_code(err_code)),
        }
    }};
}

macro_rules! lmdb_try_with_cleanup {
    ($expr:expr, $cleanup:expr) => {{
        match $expr {
            ::ffi::MDB_SUCCESS => (),
            err_code => {
                let _ = $cleanup;
                return Err(crate::Error::from_err_code(err_code));
            },
        }
    }};
}

mod cursor;
mod database;
mod environment;
mod error;
mod flags;
mod transaction;

#[cfg(test)]
mod test_utils {

    use byteorder::{ByteOrder, LittleEndian};
    use tempfile::Builder;

    use super::*;

    /// Regression test for https://github.com/danburkert/lmdb-rs/issues/21.
    /// This test reliably segfaults when run against lmbdb compiled with opt level -O3 and newer
    /// GCC compilers.
    #[test]
    fn issue_21_regression() {
        const HEIGHT_KEY: [u8; 1] = [0];

        let dir = Builder::new().prefix("test").tempdir().unwrap();

        let env = {
            let mut builder = Environment::new();
            builder.set_max_dbs(2);
            builder.set_map_size(1_000_000);
            builder.open(dir.path()).expect("open lmdb env")
        };
        let index = env.create_db(None, DatabaseFlags::DUP_SORT).expect("open index db");

        for height in 0..1000 {
            let mut value = [0u8; 8];
            LittleEndian::write_u64(&mut value, height);
            let mut tx = env.begin_rw_txn().expect("begin_rw_txn");
            tx.put(index, &HEIGHT_KEY, &value, WriteFlags::empty()).expect("tx.put");
            tx.commit().expect("tx.commit")
        }
    }
}
