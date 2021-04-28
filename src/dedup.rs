use std::cell::Cell;

thread_local! {
    static REFCOUNT: Cell<usize> = Cell::new(0);
}

pub(crate) struct Guard {
    _private: (),
}

pub(crate) fn activate() -> Guard {
    REFCOUNT.with(|refcount| refcount.set(refcount.get() + 1));
    Guard { _private: () }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let prev = REFCOUNT.with(|refcount| refcount.replace(refcount.get() - 1));
        if prev == 1 {
            crate::loc::thread_local_reset();
        }
    }
}
