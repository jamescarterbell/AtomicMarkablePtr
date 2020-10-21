use std::sync::atomic::{AtomicPtr, Ordering, AtomicUsize};
use std::mem::transmute;
use std::ops::{Deref, DerefMut, Drop};


/// A kinda bad implementation of Java's AtomicMarkableReference,
/// don't @ me if it breaks your program (use at your own risk).
#[derive(Debug)]
pub struct AtomicMarkablePtr<T>{
    ptr: AtomicPtr<T>,
}

impl<T> AtomicMarkablePtr<T>{

    /// Create a new AtomicMarkableReference with the initial marking set to mark.
    pub fn new(mut p: *mut T, mark: bool) -> Self{
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & !(0x0001)};
        p = unsafe {transmute(pu)};

        Self{
            ptr: AtomicPtr::new(p)
        }
    }

    /// Create a new AtomicMarkableReference from the raw pointer given.
    pub fn new_raw(p: *mut T) -> Self{
        Self{
            ptr: AtomicPtr::new(p)
        }
    }

    /// Get a raw mutable reference to the underlying marked pointer.
    pub fn get_mut(&mut self) -> &mut *mut T{
        self.ptr.get_mut()
    }

    /// Consume the markable reference and get the underlying pointer and
    /// the underlying mark, this seperates the mark from the pointer.
    pub fn into_inner(self) -> (*mut T, bool){
        let mut p = self.ptr.into_inner();
        let mut pu: usize = unsafe {transmute(p)};

        // Get the mark
        let mark = if (pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        pu = pu & !(0x0001);

        p = unsafe {transmute(pu)};
        (p, mark)
    }

    /// Consume the markable reference and get the underlying pointer,
    /// this doesn't seperate the mark from the pointer, so the pointer
    /// will likely be invalid/off by 1 bit.
    pub fn into_inner_raw(self) -> *mut T{
        self.ptr.into_inner()
    }

    /// Load the markable reference and get the underlying pointer and
    /// the underlying mark, this seperates the mark from the pointer.
    pub fn load(&self, order: Ordering) -> (*mut T, bool){
        let mut p = self.ptr.load(order);
        let mut pu: usize = unsafe {transmute(p)};

        // Get the mark
        let mark = if (pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        pu = pu & !(0x0001);

        p = unsafe {transmute(pu)};
        (p, mark)
    }

    /// Load the markable reference and get the underlying pointer,
    /// this doesn't seperate the mark from the pointer, so the pointer
    /// will likely be invalid/off by 1 bit.
    pub fn load_raw(&self, order: Ordering) -> *mut T{
        self.ptr.load(order)
    }

    /// Get the unmarked pointer.
    pub fn ptr(&self, order: Ordering) -> *mut T{
        self.load(order).0
    }

    /// Get the mark.
    pub fn mark(&self, order: Ordering) -> bool{
        self.load(order).1
    }

    /// Store the markable reference and set the underlying pointer and
    /// the underlying mark, this seperates the mark from the pointer.
    pub fn store(&self, mut p: *mut T, mark: bool, order: Ordering){
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & !(0x0001)};
        p = unsafe {transmute(pu)};

        self.ptr.store(p, order);
    }

    /// Store the raw pointer given, this means you need to set the mark yourself
    pub fn store_raw(&self, p: *mut T, order: Ordering){
        self.ptr.store(p, order);
    }

    /// Swap the current marked ptr with the given unmarked pointer marked by mark.
    pub fn swap(&self, mut p: *mut T, mark: bool, order: Ordering) -> (*mut T, bool){
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & !(0x0001)};
        p = unsafe {transmute(pu)};

        p = self.ptr.swap(p, order);

        pu = unsafe {transmute(p)};
        // Get the mark
        let mark = if (pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        pu = pu & !(0x0001);

        p = unsafe {transmute(pu)};
        (p, mark)
    }

    /// Swap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn swap_get_raw(&self, mut p: *mut T, mark: bool, order: Ordering) -> *mut T{
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & !(0x0001)};
        p = unsafe {transmute(pu)};

        self.ptr.swap(p, order)
    }

    /// Swap the current marked ptr with the given raw pointer. Returns the unmarked raw pointer and mark.
    pub fn swap_raw(&self, mut p: *mut T, order: Ordering) -> (*mut T, bool){

        p = self.ptr.swap(p, order);

        let mut pu: usize = unsafe {transmute(p)};
        // Get the mark
        let mark = if (pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        pu = pu & !(0x0001);

        p = unsafe {transmute(pu)};
        (p, mark)
    }

    /// Swap the current marked ptr with the given raw pointer. Returns the raw pointer.
    pub fn swap_raw_get_raw(&self, p: *mut T, order: Ordering) -> *mut T{
        self.ptr.swap(p, order)
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.
    pub fn compare_and_swap(&self, mut curr_p: *mut T, curr_mark: bool, mut new_p: *mut T, new_mark: bool, order: Ordering) -> (*mut T, bool){
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & !(0x0001)};
        new_p = unsafe {transmute(new_pu)};

        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & !(0x0001)};
        curr_p = unsafe {transmute(curr_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        new_pu = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & !(0x0001);

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn compare_and_swap_get_raw(&self, mut curr_p: *mut T, curr_mark: bool, mut new_p: *mut T, new_mark: bool, order: Ordering) -> *mut T{
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & !(0x0001)};
        new_p = unsafe {transmute(new_pu)};

        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & !(0x0001)};
        curr_p = unsafe {transmute(curr_pu)};

        self.ptr.compare_and_swap(curr_p, new_p, order)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the unmarked raw pointer and mark.
    pub fn compare_and_swap_raw(&self, mut curr_p: *mut T, curr_mark: bool, mut new_p: *mut T, order: Ordering) -> (*mut T, bool){


        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & !(0x0001)};
        curr_p = unsafe {transmute(curr_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        let mut new_pu: usize = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & !(0x0001);

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the raw pointer.
    pub fn compare_and_swap_raw_get_raw(&self, mut curr_p: *mut T, curr_mark: bool, new_p: *mut T, order: Ordering) -> *mut T{
        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & !(0x0001)};
        curr_p = unsafe {transmute(curr_pu)};

        self.ptr.compare_and_swap(curr_p, new_p, order)
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.
    pub fn raw_compare_and_swap(&self, curr_p: *mut T, mut new_p: *mut T, new_mark: bool, order: Ordering) -> (*mut T, bool){
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & !(0x0001)};
        new_p = unsafe {transmute(new_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        new_pu = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & !(0x0001);

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and wap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn raw_compare_and_swap_get_raw(&self, curr_p: *mut T, mut new_p: *mut T, new_mark: bool, order: Ordering) -> *mut T{
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & !(0x0001)};
        new_p = unsafe {transmute(new_pu)};

        self.ptr.compare_and_swap(curr_p, new_p, order)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the unmarked raw pointer and mark.
    pub fn raw_compare_and_swap_raw(&self, curr_p: *mut T, mut new_p: *mut T, order: Ordering) -> (*mut T, bool){

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        let mut new_pu: usize = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & !(0x0001);

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the raw pointer.
    pub fn raw_compare_and_swap_raw_get_raw(&self, curr_p: *mut T, new_p: *mut T, order: Ordering) -> *mut T{
        self.ptr.compare_and_swap(curr_p, new_p, order)
    }
}

pub struct AtomicMarkableArc<T>{
    ptr: AtomicMarkablePtr<ReferenceCounter<T>>,
}

pub struct ReferenceCounter<T>{
    data: T,
    pub counter: AtomicUsize,
}

impl<T> Deref for ReferenceCounter<T>{

    type Target = T;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target { 
        &self.data
     }
}

impl<T> DerefMut for ReferenceCounter<T>{

    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target { 
        &mut self.data
    }
}

impl<T> Drop for AtomicMarkableArc<T>{
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::SeqCst);
        if ptr.0 != std::ptr::null_mut(){
            let count = unsafe{(*ptr.0).counter.fetch_sub(1, Ordering::SeqCst)};
            if count == 1{
                println!("Dropping an ARC with count: {}", count);
                drop(unsafe{Box::from_raw(self.ptr.load(Ordering::SeqCst).0)})
            }
        }
    }
}

impl<T> Clone for AtomicMarkableArc<T>{  
    fn clone(&self) -> Self { 
        let ptr = self.ptr.load(Ordering::SeqCst);
        if ptr.0 != std::ptr::null_mut(){
            unsafe{(*ptr.0).counter.fetch_add(1, Ordering::SeqCst)};
        }
        AtomicMarkableArc{
            ptr: AtomicMarkablePtr::new(ptr.0, ptr.1),
        }
    }
}

impl<T> AtomicMarkableArc<T>{

    pub fn new(data: T, mark: bool) -> Self{
        let raw = Box::into_raw(Box::new(ReferenceCounter{data, counter: AtomicUsize::new(1)}));
        Self{
            ptr: AtomicMarkablePtr::new(raw, mark),
        }
    }

    pub fn null() -> Self{
        Self{
            ptr: AtomicMarkablePtr::new_raw(std::ptr::null_mut()),
        }
    }


    /// Load the markable reference and get the underlying pointer and
    /// the underlying mark, this seperates the mark from the pointer.
    pub fn load(&self, order: Ordering) -> (Option<&mut ReferenceCounter<T>>, bool){
        let p = self.ptr.load(order);

        match p.0 == std::ptr::null_mut(){
            true => (None, p.1),
            false => (Some(unsafe{p.0.as_mut().unwrap()}), p.1)
        }
    }

    /// Get the mark.
    pub fn mark(&self, order: Ordering) -> bool{
        self.load(order).1
    }

    /// Store the markable reference and set the underlying pointer and
    /// the underlying mark, this seperates the mark from the pointer.
    pub fn store(&self, ptr: AtomicMarkableArc<T>, mark: bool, order: Ordering){
        let p = ptr.ptr.load(Ordering::SeqCst);

        let old = self.ptr.swap(p.0 as *mut ReferenceCounter<T>, mark, order);
        if old.0 != p.0 && old.0 != std::ptr::null_mut(){
            unsafe{(*old.0).counter.fetch_sub(1, Ordering::SeqCst)};
        }
        //We need to increase the reference count before the Arc gets dropped
        unsafe{(*p.0).counter.fetch_add(1, Ordering::SeqCst)};
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.
    pub fn compare_and_swap(&self, curr_ptr: AtomicMarkableArc<T>, curr_mark: bool, new_ptr: AtomicMarkableArc<T>, new_mark: bool, order: Ordering) -> (Option<&mut ReferenceCounter<T>>, bool){
        let p = new_ptr.ptr.load(Ordering::SeqCst);
        let curr_p = curr_ptr.ptr.load(Ordering::SeqCst);

        let new_p = self.ptr.compare_and_swap(curr_p.0, curr_mark, p.0, new_mark, order);

        if (curr_p.0, curr_mark) == (new_p.0, new_mark){
            if p.0 != std::ptr::null_mut(){
                unsafe{(*p.0).counter.fetch_add(1, Ordering::SeqCst)};
            }
            if curr_p.0 != std::ptr::null_mut(){
                unsafe{(*curr_p.0).counter.fetch_sub(1, Ordering::SeqCst)};
            }
        };

        match unsafe{new_p.0.as_mut()}{
            Some(ptr) => (Some(ptr), new_p.1),
            None => (None, p.1),
        }
    }
}

impl<T> Eq for &AtomicMarkableArc<T>{}

impl<T> PartialEq for &AtomicMarkableArc<T>{
    fn eq(&self, r: &Self) -> bool{
        self.ptr.load(Ordering::SeqCst).0 == r.ptr.load(Ordering::SeqCst).0
    }

    fn ne(&self, other: &Self) -> bool{
        !self.eq(other)
    }
}

impl<T> Eq for &ReferenceCounter<T>{}

impl<T> PartialEq for &ReferenceCounter<T>{

    fn eq(&self, r: &Self) -> bool { 
        *self as *const ReferenceCounter<T> == *r as *const ReferenceCounter<T>
     }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T> Eq for &mut AtomicMarkableArc<T>{}

impl<T> PartialEq for &mut AtomicMarkableArc<T>{
    fn eq(&self, r: &Self) -> bool{
        self.ptr.load(Ordering::SeqCst).0 == r.ptr.load(Ordering::SeqCst).0
    }

    fn ne(&self, other: &Self) -> bool{
        !self.eq(other)
    }
}

impl<T> Eq for &mut ReferenceCounter<T>{}

impl<T> PartialEq for &mut ReferenceCounter<T>{

    fn eq(&self, r: &Self) -> bool { 
        *self as *const ReferenceCounter<T> == *r as *const ReferenceCounter<T>
     }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub enum PtrErrors{
    NullPtrError,
}

#[cfg(test)]
mod tests{
    use super::{AtomicMarkablePtr, AtomicMarkableArc, Ordering};

    #[test]
    fn create_ptr(){
        assert_eq!(std::ptr::null_mut(), AtomicMarkablePtr::<usize>::new(std::ptr::null_mut(), false).load_raw(Ordering::SeqCst));
        assert_ne!(std::ptr::null_mut(), AtomicMarkablePtr::<usize>::new(std::ptr::null_mut(), true).load_raw(Ordering::SeqCst));
        assert_eq!(std::ptr::null_mut(), AtomicMarkablePtr::<usize>::new(std::ptr::null_mut(), false).load(Ordering::SeqCst).0);
        assert_eq!(std::ptr::null_mut(), AtomicMarkablePtr::<usize>::new(std::ptr::null_mut(), true).load(Ordering::SeqCst).0);
    }

    #[test]
    fn create_arcs(){
        let ptr = AtomicMarkableArc::<usize>::null();
        assert_eq!(true , ptr.load(Ordering::SeqCst).0.is_none());

        let new_ptr = AtomicMarkableArc::new(5, true);
        ptr.store( new_ptr.clone(), false, Ordering::SeqCst);
        let val = ptr.load(Ordering::SeqCst);
        assert_eq!(true, val.0.is_some());
        assert_eq!(true, val.0.unwrap().data == 5);

        let newer_ptr = AtomicMarkableArc::new(20, true);
        ptr.compare_and_swap(new_ptr, false, newer_ptr,true, Ordering::SeqCst);
        let val = ptr.load(Ordering::SeqCst);
        assert_eq!(true, val.0.is_some());
        assert_eq!(true, val.0.unwrap().data == 20);
    }

    #[test]
    fn create_ptr_non_null(){
        let ptr_1 = Box::into_raw(Box::new(5));
        assert_eq!(ptr_1, AtomicMarkablePtr::<usize>::new(ptr_1, false).load_raw(Ordering::SeqCst));
        //assert_ne!(ptr_1, AtomicMarkablePtr::<usize>::new(ptr_1, true).load_raw(Ordering::SeqCst));
        //assert_eq!(ptr_1, AtomicMarkablePtr::<usize>::new(ptr_1, false).load(Ordering::SeqCst).0);
        //assert_eq!(ptr_1, AtomicMarkablePtr::<usize>::new(ptr_1, true).load(Ordering::SeqCst).0);
    }
}