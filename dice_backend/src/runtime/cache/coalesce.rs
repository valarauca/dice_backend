use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::super::super::seahasher::DefaultSeaHasher;
use super::super::{Iter, Rational};

thread_local! {
    static ANY_CACHE: RefCell<HashMap<(Iter,Iter,TypeId),Iter,DefaultSeaHasher>> = RefCell::new(HashMap::default());
}

pub fn check<T>(i1: Iter, i2: Iter) -> Result<Iter, ((Iter, Iter), (Iter, Iter))>
where
    T: 'static + ?Sized,
{
    ANY_CACHE.with(|cache| -> Result<Iter, ((Iter, Iter), (Iter, Iter))> {
        let id = TypeId::of::<T>();
        let tuple = (i1, i2, id);
        match cache.borrow().deref().get(&tuple) {
            Option::Some(item) => Ok(item.clone()),
            Option::None => {
                let tup = (tuple.0, tuple.1);
                Err((tup.clone(), tup))
            }
        }
    })
}

pub fn insert<T>(arg: (Iter, Iter), output: Iter)
where
    T: 'static + ?Sized,
{
    ANY_CACHE.with(|cache| -> () {
        let id = TypeId::of::<T>();
        let tuple = (arg.0, arg.1, id);
        cache.borrow_mut().deref_mut().insert(tuple, output);
        ()
    });
}
