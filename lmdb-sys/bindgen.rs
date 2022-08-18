use bindgen::callbacks::IntKind;
use bindgen::callbacks::ParseCallbacks;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "MDB_SUCCESS"
            | "MDB_KEYEXIST"
            | "MDB_NOTFOUND"
            | "MDB_PAGE_NOTFOUND"
            | "MDB_CORRUPTED"
            | "MDB_PANIC"
            | "MDB_VERSION_MISMATCH"
            | "MDB_INVALID"
            | "MDB_MAP_FULL"
            | "MDB_DBS_FULL"
            | "MDB_READERS_FULL"
            | "MDB_TLS_FULL"
            | "MDB_TXN_FULL"
            | "MDB_CURSOR_FULL"
            | "MDB_PAGE_FULL"
            | "MDB_MAP_RESIZED"
            | "MDB_INCOMPATIBLE"
            | "MDB_BAD_RSLOT"
            | "MDB_BAD_TXN"
            | "MDB_BAD_VALSIZE"
            | "MDB_BAD_DBI"
            | "MDB_LAST_ERRCODE" => Some(IntKind::Int),
            "MDB_SIZE_MAX" | "MDB_PROBLEM" => Some(IntKind::U64),
            _ => Some(IntKind::UInt),
        }
    }
}

pub fn generate() {
    let lmdb_h = match std::env::var("LMDB_H_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => match pkg_config::probe_library("lmdb") {
            Ok(mut library) => library.include_paths.pop().unwrap().join("lmdb.h"),
            Err(_) => {
                let lmdb = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
                lmdb.join("lmdb")
                    .join("libraries")
                    .join("liblmdb")
                    .join("lmdb.h")
            }
        },
    };

    let out_bindings_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    let bindings = bindgen::Builder::default()
        .header(lmdb_h.to_string_lossy())
        .allowlist_var("^(MDB|mdb)_.*")
        .allowlist_type("^(MDB|mdb)_.*")
        .allowlist_function("^(MDB|mdb)_.*")
        .size_t_is_usize(true)
        .ctypes_prefix("::libc")
        .blocklist_item("mode_t")
        .blocklist_item("mdb_mode_t")
        .blocklist_item("mdb_filehandle_t")
        .blocklist_item("^__.*")
        .parse_callbacks(Box::new(Callbacks {}))
        .layout_tests(false)
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&out_bindings_path)
        .expect("Couldn't write bindings!");
}
