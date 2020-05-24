use std::os::raw::c_void;
use std::os::raw::c_int;
use libloading::Library;
use libloading::Error;
use libloading::Symbol;
use lazy_static::lazy_static;

type LockFunctionType<'a> = Symbol<'a, unsafe extern fn(*const c_void) -> c_int>;
type UnLockFunctionType<'a> = Symbol<'a, unsafe extern fn(*const c_void) -> c_int>;

lazy_static! {
    static ref ORIG_PTHREAD_LIBRARY: Library = {
        let lib: Result<Library, Error> = Library::new("libpthread.so");
        match lib {
            Err(_) => panic!("Failed to load libpthread.so"),
            Ok(r) => r,
        }
    };

    static ref ORIG_PTHREAD_MUTEX_LOCK: LockFunctionType<'static> = {
        unsafe {
            let func: Result<LockFunctionType, Error> = ORIG_PTHREAD_LIBRARY.get(b"pthread_mutex_lock");
            match func {
                Err(_) => panic!("Failed to load pthread_mutex_lock"),
                Ok(r) => r,
            }
        }
    };

    static ref ORIG_PTHREAD_MUTEX_UNLOCK: UnLockFunctionType<'static> = {
        unsafe {
            let func: Result<UnLockFunctionType, Error> = ORIG_PTHREAD_LIBRARY.get(b"pthread_mutex_unlock");
            match func {
                Err(_) => panic!("Failed to load pthread_mutex_unlock"),
                Ok(r) => r,
            }
        }
    };
}

pub fn orig_pthread_mutex_lock(mutex: *const c_void) -> c_int {
    unsafe {
        ORIG_PTHREAD_MUTEX_LOCK(mutex)
    }
}

pub fn orig_pthread_mutex_unlock(mutex: *const c_void) -> c_int {
    unsafe {
        ORIG_PTHREAD_MUTEX_UNLOCK(mutex)
    }
}
