use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use crate::{from_io, IoDeSer};

impl <'b,T> IoDeSer<'b> for Rc<T>
where
    T: IoDeSer<'b>
{
    type Output = Rc<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Rc::new(from_io!(io_input, T)?))
    }
}


impl <'a, T> IoDeSer<'a> for Arc<T>
    where T: IoDeSer<'a>
{
    type Output = Arc<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Arc::new(from_io!(io_input, T)?))
    }
}


impl <'a,T> IoDeSer<'a> for Box<T>
    where T: IoDeSer<'a>
{
    type Output = Box<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Box::new(from_io!(io_input, T)?))
    }
}

impl <'a ,T> IoDeSer<'a> for Mutex<T>
    where T: IoDeSer<'a>
{
    type Output = Mutex<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.lock().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Mutex::new(from_io!(io_input, T)?))
    }
}

impl <'a,T> IoDeSer<'a> for RefCell<T>
    where T: IoDeSer<'a>
{
    type Output = RefCell<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.borrow().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(RefCell::new(from_io!(io_input, T)?))
    }
}

impl <'a,T> IoDeSer<'a> for Cell<T>
    where T: IoDeSer<'a> + Copy
{
    type Output = Cell<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.get().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Cell::new(from_io!(io_input, T)?))
    }
}

impl <'a,T> IoDeSer<'a> for Weak<T>
    where T: IoDeSer<'a>
{
    type Output = Weak<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.upgrade().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(Rc::downgrade(&Rc::new(from_io!(io_input, T)?)))
    }
}

impl <'a,T> IoDeSer<'a> for RwLock<T>
    where T: IoDeSer<'a>
{
    type Output = RwLock<T::Output>;

    fn to_io_string(&self, tab: u8) -> String {
        self.read().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self::Output> where Self: Sized {
        Ok(RwLock::new(from_io!(io_input, T)?))
    }
}
