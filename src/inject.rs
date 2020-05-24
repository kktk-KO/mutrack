mod activate;
mod pthread;
mod recorder;
mod debug;
mod message;

use std::os::raw::c_void;
use std::os::raw::c_int;
use std::time::Instant;
use nix::unistd::gettid;
use scopeguard::defer;
use crate::activate::try_activate;
use crate::pthread::orig_pthread_mutex_lock;
use crate::pthread::orig_pthread_mutex_unlock;
use crate::recorder::ensure_recorder;
use crate::recorder::record;
use crate::message::Message;
use crate::message::MessageType;

#[no_mangle]
pub extern fn pthread_mutex_lock(mutex: *const c_void) -> c_int {
    try_activate(
        mutex,
        |mutex|{
            ensure_recorder();
            let tid = gettid().as_raw();
            let start_time = Instant::now();
            defer!{
                record(
                    Message{
                        start_time: start_time,
                        finish_time: Instant::now(),
                        tid: tid,
                        message_type: MessageType::Lock
                    }
                )
            };
            orig_pthread_mutex_lock(mutex)
        },
        |mutex|{
            orig_pthread_mutex_lock(mutex)
        }
    )
}

#[no_mangle]
pub extern fn pthread_mutex_unlock(mutex: *const c_void) -> c_int {
    try_activate(
        mutex,
        |mutex|{
            ensure_recorder();
            let tid = gettid().as_raw();
            let start_time = Instant::now();
            defer!{
                record(
                    Message{
                        start_time: start_time,
                        finish_time: Instant::now(),
                        tid: tid,
                        message_type: MessageType::UnLock
                    }
                )
            };
            orig_pthread_mutex_unlock(mutex)
        },
        |mutex|{
            orig_pthread_mutex_unlock(mutex)
        }
    )
}
