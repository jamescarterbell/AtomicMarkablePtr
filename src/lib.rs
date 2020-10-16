use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::transmute;

/// A kinda bad implementation of Java's AtomicMarkableReference,
/// don't @ me if it breaks your program (use at your own risk).
pub struct AtomicMarkablePtr<T>{
    ptr: AtomicPtr<T>,
}

impl<T> AtomicMarkablePtr<T>{

    /// Create a new AtomicMarkableReference with the initial marking set to mark.
    pub fn new(mut p: *mut T, mark: bool) -> Self{
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & 0xFFFE};
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
        pu = pu & 0xFFFE;

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
        pu = pu & 0xFFFE;

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
        pu = if mark {pu | 0x0001} else {pu & 0xFFFE};
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
        pu = if mark {pu | 0x0001} else {pu & 0xFFFE};
        p = unsafe {transmute(pu)};

        p = self.ptr.swap(p, order);

        pu = unsafe {transmute(p)};
        // Get the mark
        let mark = if (pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        pu = pu & 0xFFFE;

        p = unsafe {transmute(pu)};
        (p, mark)
    }

    /// Swap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn swap_get_raw(&self, mut p: *mut T, mark: bool, order: Ordering) -> *mut T{
        let mut pu: usize = unsafe {transmute(p)};
        pu = if mark {pu | 0x0001} else {pu & 0xFFFE};
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
        pu = pu & 0xFFFE;

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
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & 0xFFFE};
        new_p = unsafe {transmute(new_pu)};

        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & 0xFFFE};
        curr_p = unsafe {transmute(curr_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        new_pu = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & 0xFFFE;

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn compare_and_swap_get_raw(&self, mut curr_p: *mut T, curr_mark: bool, mut new_p: *mut T, new_mark: bool, order: Ordering) -> *mut T{
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & 0xFFFE};
        new_p = unsafe {transmute(new_pu)};

        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & 0xFFFE};
        curr_p = unsafe {transmute(curr_pu)};

        self.ptr.compare_and_swap(curr_p, new_p, order)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the unmarked raw pointer and mark.
    pub fn compare_and_swap_raw(&self, mut curr_p: *mut T, curr_mark: bool, mut new_p: *mut T, order: Ordering) -> (*mut T, bool){


        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & 0xFFFE};
        curr_p = unsafe {transmute(curr_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        let mut new_pu: usize = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & 0xFFFE;

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the raw pointer.
    pub fn compare_and_swap_raw_get_raw(&self, mut curr_p: *mut T, curr_mark: bool, new_p: *mut T, order: Ordering) -> *mut T{
        let mut curr_pu: usize = unsafe {transmute(curr_p)};
        curr_pu = if curr_mark {curr_pu | 0x0001} else {curr_pu & 0xFFFE};
        curr_p = unsafe {transmute(curr_pu)};

        self.ptr.compare_and_swap(curr_p, new_p, order)
    }

    /// Compare and swap the current marked ptr with the given unmarked pointer marked by mark.
    pub fn raw_compare_and_swap(&self, curr_p: *mut T, mut new_p: *mut T, new_mark: bool, order: Ordering) -> (*mut T, bool){
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & 0xFFFE};
        new_p = unsafe {transmute(new_pu)};

        new_p = self.ptr.compare_and_swap(curr_p, new_p, order);

        new_pu = unsafe {transmute(new_p)};
        // Get the mark
        let mark = if (new_pu & 0x0001) == 1 {true} else {false};

        // Reverse the mark
        new_pu = new_pu & 0xFFFE;

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and wap the current marked ptr with the given unmarked pointer marked by mark.  Returns the raw previous pointer.
    pub fn raw_compare_and_swap_get_raw(&self, curr_p: *mut T, mut new_p: *mut T, new_mark: bool, order: Ordering) -> *mut T{
        let mut new_pu: usize = unsafe {transmute(new_p)};
        new_pu = if new_mark {new_pu | 0x0001} else {new_pu & 0xFFFE};
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
        new_pu = new_pu & 0xFFFE;

        new_p = unsafe {transmute(new_pu)};
        (new_p, mark)
    }

    /// Compare and swap the current marked ptr with the given raw pointer. Returns the raw pointer.
    pub fn raw_compare_and_swap_raw_get_raw(&self, curr_p: *mut T, new_p: *mut T, order: Ordering) -> *mut T{
        self.ptr.compare_and_swap(curr_p, new_p, order)
    }
}