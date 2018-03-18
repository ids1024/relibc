use core::nonzero::Zeroable;

pub struct NullTermArrayIter<T:  Zeroable + Copy> {
    ptr: *const T,
}

impl<T: Zeroable + Copy> NullTermArrayIter<T> {
    pub unsafe fn new(ptr: *const T) -> Self {
        Self { ptr }
    }
}

impl<T: Zeroable + Copy> Iterator for NullTermArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = unsafe { *self.ptr };
        if value.is_zero() {
            None
        } else {
            self.ptr = unsafe { self.ptr.offset(1) };
            Some(value)
        }
    }
}

#[test]
fn test_nulltermarray_strings() {
    use core::ptr;
    use alloc::Vec;

    let strlist = vec!["array\0", "of\0", "strings\0"];
    let mut ptrlist: Vec<_> = strlist.iter().map(|x| x.as_ptr()).collect();
    ptrlist.push(ptr::null());
    
    let mut iter = unsafe { NullTermArrayIter::new(ptrlist.as_ptr()) };
    for i in strlist.iter() {
        assert_eq!(iter.next(), Some(i.as_ptr()));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_nulltermarray_string() {
    let text = "string\0";
    let mut iter = unsafe { NullTermArrayIter::new(text.as_ptr()) };
    for i in text[..text.len()-1].as_bytes().iter() {
        assert_eq!(iter.next(), Some(*i));
    }
    assert_eq!(iter.next(), None);
}
