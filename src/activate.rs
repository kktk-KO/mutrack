use std::cell::Cell;
use std::thread_local;
use scopeguard::defer;

thread_local! {
    static IS_ACTIVE: Cell<bool> = Cell::new(false);
}

pub fn try_activate<T, F, E, R>(t: T, f: F, e: E) -> R
    where F: FnOnce(T) -> R,
          E: FnOnce(T) -> R,
{
    IS_ACTIVE.with(|is_active| -> R {
        if !is_active.get() {
            is_active.set(true);
            defer!({ is_active.set(false); });
            f(t)
        } else {
            e(t)
        }
    })
}

pub fn activate() {
    IS_ACTIVE.with(|is_active| {
        is_active.set(true);
    });
}
