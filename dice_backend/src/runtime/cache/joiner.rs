use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::super::super::seahasher::DefaultSeaHasher;
use super::super::{Iter, Rational};

thread_local! {
    static JOIN_CACHE: RefCell<HashMap<(Iter,Iter),Iter,DefaultSeaHasher>> = RefCell::new(HashMap::default());
}

pub fn check(i1: Iter, i2: Iter) -> Result<Iter, ((Iter, Iter), (Iter, Iter))> {
    JOIN_CACHE.with(|cache| -> Result<Iter, ((Iter, Iter), (Iter, Iter))> {
        let tuple = (i1, i2);
        match cache.borrow().deref().get(&tuple) {
            Option::Some(item) => Ok(item.clone()),
            Option::None => Err((tuple.clone(), tuple)),
        }
    })
}

pub fn insert(arg: (Iter, Iter), output: &Iter) {
    JOIN_CACHE.with(|cache| -> () {
        cache
            .borrow_mut()
            .deref_mut()
            .insert(arg.clone(), output.clone());
        ()
    });

    /*
     * This is associative so we can swap args and get the same
     * result
     *
     */
    let other = (arg.1, arg.0);
    JOIN_CACHE.with(|cache| -> () {
        cache.borrow_mut().deref_mut().insert(other, output.clone());
        ()
    });
}
