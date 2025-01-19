use std::alloc::{self, Layout};
use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

use crate::iterator::{IntoIteratorII, Iterator};

#[allow(dead_code)]
#[derive(Debug)]
pub struct VecII<T> {
    ptr: NonNull<T>,
    size: usize,
    capacity: usize,
}

unsafe impl<T: Send> Send for VecII<T> {}
unsafe impl<T: Sync> Sync for VecII<T> {}

impl<T> Default for VecII<T> {
    fn default() -> Self {
        VecII {
            ptr: NonNull::dangling(),
            size: 0,
            capacity: 0,
        }
    }
}

#[allow(dead_code)]
impl<T> VecII<T> {
    pub fn new() -> VecII<T> {
        VecII {
            ptr: NonNull::dangling(),
            size: 0,
            capacity: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.capacity;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_cap;
    }

    pub fn push(&mut self, value: T) {
        if self.size == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len()), value);
        }

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.size))) }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.size, "index out of bould");
        if self.size == self.capacity {
            self.grow()
        }

        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.size - index,
            );

            ptr::write(self.ptr.as_ptr().add(index), value);
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.size, "index out of bound");

        unsafe {
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.size - index,
            );
            self.size -= 1;
            result
        }
    }
}

impl<T> Deref for VecII<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len()) }
    }
}

impl<T> DerefMut for VecII<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len()) }
    }
}

impl<T> Drop for VecII<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            loop {
                let item = self.pop();
                if item.is_none() {
                    break;
                }
            }

            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[allow(dead_code)]
pub struct IntoIter<T> {
    buf: NonNull<T>,
    capacity: usize,
    start: *const T,
    end: *const T,
}

impl<T> IntoIteratorII<T> for VecII<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iterII(self) -> Self::IntoIter {
        let vec = ManuallyDrop::new(self);

        let ptr = vec.ptr;
        let capacity = vec.capacity;
        let len = vec.size;

        IntoIter {
            buf: ptr,
            capacity,
            start: ptr.as_ptr(),
            end: if capacity == 0 {
                ptr.as_ptr()
            } else {
                unsafe { ptr.as_ptr().add(len) }
            },
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Iterator<T> for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len() {
        let mut list = VecII::<i32>::new();
        assert_eq!(list.len(), 0);

        list.push(23);
        assert_eq!(list.len(), 1);

        assert_eq!(list.pop().unwrap(), 23);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_capacity() {
        let mut list = VecII::<i32>::new();
        assert_eq!(list.capacity, 0);

        list.grow();
        assert_eq!(list.capacity, 1);

        list.grow();
        assert_eq!(list.capacity, 2);

        list.grow();
        assert_eq!(list.capacity, 4);
    }

    #[test]
    fn test_iterator() {
        let mut list = VecII::<i32>::new();
        list.push(22);
        list.push(33);
        list.push(44);

        let mut it = list.into_iterII();
        assert_eq!(it.next(), Some(22));
        assert_eq!(it.next(), Some(33));
        assert_eq!(it.next(), Some(44));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_deref() {
        let mut list = VecII::<i32>::new();
        list.push(22);

        for _ in &(*list) {}
    }

    #[test]
    fn test_insert_remove() {
        let mut list = VecII::<i32>::new();
        list.push(1);
        list.push(2);
        list.insert(1, 22);
        list.insert(2, 23);

        assert_eq!(list.remove(1), 22);
        assert_eq!(list.remove(1), 23);
        assert_eq!(list.remove(0), 1);
    }
}
