use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::super::super::seahasher::DefaultSeaHasher;
use super::super::{Dice6, Iter, Rational};

thread_local! {
    static DICE6_CACHE: RefCell<HashMap<(usize,Rational),Iter,DefaultSeaHasher>> = RefCell::new(HashMap::default());
}

pub fn check_value(arg: (usize, Rational)) -> Option<Iter> {
    DICE6_CACHE.with(|cache| -> Option<Iter> {
        match cache.borrow().deref().get(&arg) {
            Option::Some(item) => Some(item.clone()),
            Option::None => None,
        }
    })
}

pub fn insert_value(arg: (usize, Rational), output: Iter) {
    DICE6_CACHE.with(|cache| -> () {
        cache.borrow_mut().deref_mut().insert(arg, output);
        ()
    });
}
