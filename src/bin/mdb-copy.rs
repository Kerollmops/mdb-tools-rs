use std::ffi::CString;
use std::fs::File;
use std::os::unix::{ffi::OsStrExt, io::AsRawFd};
use std::path::PathBuf;
use std::ptr;

use lmdb_sys::{MDB_env, mdb_env_create, mdb_env_open, mdb_env_close, mdb_env_copyfd2};
use libc::c_uint;
use main_error::MainError;

use mdb_tools_rs::lmdb_result;

fn main() -> Result<(), MainError> {
    unsafe {
        let mut env: *mut MDB_env = ptr::null_mut();
        lmdb_result(mdb_env_create(&mut env))?;

        let arg = std::env::args().nth(1).expect("src path missing");
        let path = PathBuf::from(arg);
        let path = CString::new(path.as_os_str().as_bytes())?;

        let result = lmdb_result(mdb_env_open(env, path.as_ptr(), lmdb_sys::MDB_RDONLY, 0o400));

        let file = File::create("out.mdb")?;
        let fd = file.as_raw_fd();

        let ret = mdb_env_copyfd2(env, fd, lmdb_sys::MDB_CP_COMPACT);

        println!("ret: {:?}", ret);

        mdb_env_close(env);
    }

    Ok(())
}
