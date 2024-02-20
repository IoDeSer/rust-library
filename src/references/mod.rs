use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use crate::{from_io, IoDeSer};

impl <T> IoDeSer for Rc<T>
where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Rc::new(from_io!(io_input, T)?))
    }
}


impl <T> IoDeSer for Arc<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Arc::new(from_io!(io_input, T)?))
    }
}


impl <T> IoDeSer for Box<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.as_ref().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Box::new(from_io!(io_input, T)?))
    }
}

impl <T> IoDeSer for Mutex<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.lock().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Mutex::new(from_io!(io_input, T)?))
    }
}

impl <T> IoDeSer for RefCell<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.borrow().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(RefCell::new(from_io!(io_input, T)?))
    }
}

impl <T> IoDeSer for Cell<T>
    where T: IoDeSer + Copy
{
    fn to_io_string(&self, tab: u8) -> String {
        self.get().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Cell::new(from_io!(io_input, T)?))
    }
}

impl <T> IoDeSer for Weak<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.upgrade().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(Rc::downgrade(&Rc::new(from_io!(io_input, T)?)))
    }
}

impl <T> IoDeSer for RwLock<T>
    where T: IoDeSer
{
    fn to_io_string(&self, tab: u8) -> String {
        self.read().unwrap().to_io_string(tab)
    }

    fn from_io_string(io_input: &mut String) -> crate::Result<Self> where Self: Sized {
        Ok(RwLock::new(from_io!(io_input, T)?))
    }
}
