use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::super::super::seahasher::DefaultSeaHasher;
use super::super::{Iter, Rational};

thread_local! {
    static CHAIN_CACHE: RefCell<HashMap<(Iter,TypeId),Iter,DefaultSeaHasher>> = RefCell::new(HashMap::default());
}

pub fn check<T>(i1: Iter) -> Result<Iter, (Iter, Iter)>
where
    T: 'static + ?Sized,
{
    CHAIN_CACHE.with(|cache| -> Result<Iter, (Iter, Iter)> {
        let id = TypeId::of::<T>();
        let tuple = (i1, id);
        match cache.borrow().deref().get(&tuple) {
            Option::Some(item) => Ok(item.clone()),
            Option::None => {
                let arg = tuple.0;
                Err((arg.clone(), arg))
            }
        }
    })
}

pub fn insert<T>(arg: Iter, output: &Iter)
where
    T: 'static + ?Sized,
{
    CHAIN_CACHE.with(|cache| -> () {
        let id = TypeId::of::<T>();
        let tuple = (arg, id);
        cache.borrow_mut().deref_mut().insert(tuple, output.clone());
        ()
    });
}
