//! CowArc can be useful for decreasing memory allocations by sharing immutable memory.
//! It saves some RAM by sharing immutable values between CowArc clones.
//! Memory allocates only in case of changing value.
//! CowArc can be usefull for creating builders.

use std::{
    sync::{
        Arc
    },
    ops::{
        Deref
    }
};

#[derive(Debug)]
pub struct CowArc<T: Clone>{
    inner: Arc<T>
}
impl<T: Clone> CowArc<T> {
    /// Creates new CowArc value
    pub fn new(val: T) -> CowArc<T>{
        CowArc{
            inner: Arc::new(val)
        }
    }
    
    /// Method sets new value for inner Arc value.
    /// Performs new allocation. 
    /// All previous values are still available over previous clones.
    /// # Examples
    /// ```
    /// use cow_arc::CowArc;
    /// use std::ops::Deref;
    ///
    /// let source = CowArc::new(vec![1, 2, 3]);
    /// 
    /// // Still shared memory
    /// let mut changed = source.clone();
    /// assert!(std::ptr::eq(source.deref(), changed.deref()) == true);
    /// assert!(changed.eq(&vec![1, 2, 3]));
    /// 
    /// // New memory allocation
    /// changed.set_val(vec![1, 2, 3, 4]);
    /// assert!(std::ptr::eq(source.deref(), changed.deref()) == false);
    /// assert!(changed.eq(&vec![1, 2, 3, 4]));
    /// ```
    pub fn set_val(&mut self, val: T){
        self.inner = Arc::new(val);
    }

    /// Method updates inner Arc value by replacing it with new value.
    /// Performs new allocation.
    /// All previous values are still available over previous clones.
    /// # Examples
    /// ```
    /// use cow_arc::CowArc;
    /// use std::ops::Deref;
    ///
    /// let source = CowArc::new(vec![1, 2, 3]);
    /// 
    /// // Still shared memory
    /// let mut updated = source.clone();
    /// assert!(std::ptr::eq(source.deref(), updated.deref()) == true);
    /// assert!(updated.eq(&vec![1, 2, 3]));
    /// 
    /// // New memory allocation
    /// updated.update_val(|val|{
    ///        val.push(4);
    /// });
    /// assert!(std::ptr::eq(source.deref(), updated.deref()) == false);
    /// assert!(updated.eq(&vec![1, 2, 3, 4]));
    /// ```
    pub fn update_val<F: FnOnce(&mut T)>(&mut self, f: F) {
        let mut v: T = self.inner.deref().clone();
        f(&mut v);
        self.inner = Arc::new(v);
    }
}
impl<T: Clone> Deref for CowArc<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<T: Clone> Clone for CowArc<T>{
    fn clone(&self) -> Self {
        CowArc{
            inner: self.inner.clone()
        }   
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_cow_arc(){
        {
            let test_str = "Test string";
            let new_val_str = "New value";

            let source = CowArc::new(test_str.to_owned());
            let cloned = source.clone();
            let mut changed = cloned.clone();
            changed.set_val(new_val_str.to_owned());
            let changed_cloned = changed.clone();
    
            let source_ptr: &String = source.deref();
            let cloned_ptr: &String = cloned.deref();
            let changed_ptr: &String = changed.deref();
            let changed_cloned_ptr: &String = changed_cloned.deref();
    
            assert!(std::ptr::eq(source_ptr, cloned_ptr));
            assert!(std::ptr::eq(source_ptr, changed_ptr) == false);
            assert!(std::ptr::eq(changed_ptr, changed_cloned_ptr));
            assert!(cloned_ptr.eq(test_str));
            assert!(changed.eq(new_val_str));
        }

        {
            let source = CowArc::new(vec![1, 2, 3]);
            let cloned = source.clone();
            let mut changed = cloned.clone();
            changed.set_val(vec![1, 2, 3, 4]);
            let changed_cloned = changed.clone();
            let mut updated = changed_cloned.clone();
            updated.update_val(|val|{
                val.push(5);
            });

            let source_ptr: &Vec<i32> = source.deref();
            let cloned_ptr: &Vec<i32> = cloned.deref();
            let changed_ptr: &Vec<i32> = changed.deref();
            let changed_cloned_ptr: &Vec<i32> = changed_cloned.deref();
            let updated_ptr: &Vec<i32> = updated.deref();

            assert!(std::ptr::eq(source_ptr, cloned_ptr));
            assert!(std::ptr::eq(source_ptr, changed_ptr) == false);
            assert!(std::ptr::eq(changed_ptr, changed_cloned_ptr));
            assert!(std::ptr::eq(changed_ptr, updated_ptr) == false);
            assert!(changed.eq(&vec![1, 2, 3, 4]));
            assert!(updated.eq(&vec![1, 2, 3, 4, 5]));
        }
    }
}