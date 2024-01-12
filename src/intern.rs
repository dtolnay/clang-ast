use rustc_hash::FxHashSet as HashSet;
use serde::de::{DeserializeSeed, Deserializer, Error, Visitor};
use std::cell::{Cell, RefCell};
use std::fmt;
use std::sync::Arc;

thread_local! {
    static REFCOUNT: Cell<usize> = const { Cell::new(0) };
    static INTERN: RefCell<HashSet<Arc<str>>> = RefCell::new(HashSet::default());
}

fn borrowed(string: &str) -> Arc<str> {
    do_intern(string)
}

fn owned(string: String) -> Arc<str> {
    do_intern(string)
}

fn do_intern(string: impl AsRef<str> + Into<Arc<str>>) -> Arc<str> {
    INTERN.with(|intern| {
        let mut intern = intern.borrow_mut();
        if let Some(arc) = intern.get(string.as_ref()) {
            Arc::clone(arc)
        } else {
            let arc: Arc<str> = string.into();
            intern.insert(Arc::clone(&arc));
            arc
        }
    })
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
            INTERN.with(|intern| intern.borrow_mut().clear());
        }
    }
}

pub(crate) struct InternVisitor;

impl<'de> Visitor<'de> for InternVisitor {
    type Value = Arc<str>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(borrowed(string))
    }

    fn visit_string<E>(self, string: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(owned(string))
    }
}

impl<'de> DeserializeSeed<'de> for InternVisitor {
    type Value = Arc<str>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}
