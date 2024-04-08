use std::cell::RefCell;
use std::marker::Copy;
pub(crate) struct Cache<T> 
where T: Copy, 
{
    data: RefCell<Option<T>>,
    is_dirty: RefCell<bool>,
    computed: Box<dyn Fn()->T>,
}

impl<T> Cache<T>
where T: Copy, 
{
    pub fn new<F>(computed: F) -> Self
    where
        F: 'static + Fn() -> T,
        T: Copy,
    {
        Cache {
            data: RefCell::new(None),
            is_dirty: RefCell::new(true),
            computed: Box::new(computed),
        }
    }

    pub fn value(&self) -> T {
        if *self.is_dirty.borrow() {
            self.data.replace(Some((self.computed)()));
            self.is_dirty.replace(false);
        }
        match *self.data.borrow() {
            Some(data) => data,
            _ => unreachable!(),
        }
    }
}
