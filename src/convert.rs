use crate::util::vec_push;
use std::mem;

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs {
	($($id:ident = $expr:expr);+$(;)?) => {
		$(
			let $id = $expr;
			let $id = unsafe{ $crate::convert::static_ref(&$id) };
		)+
	}
}

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs_mut {
	($($id:ident = $expr:expr);+$(;)?) => {
		$(
			let mut $id = $expr;
			let $id = unsafe{ $crate::convert::static_ref_mut(&mut $id) };
		)+
	}
}

#[macro_export]
macro_rules! init_static_array {
    ($ele:expr, $ele_size:expr, $arr_size:expr) => {{
        use std::intrinsics::transmute;
        use std::mem::size_of;

        let copiable: [u8; $ele_size] = transmute($ele);
        transmute([copiable; $arr_size])
    }};
}

pub struct StaticRefArray<T>(Vec<T>);

impl<T> StaticRefArray<T> {
    pub fn new(len: usize, constructor: impl FnMut() -> T) -> Self {
        Self(vec_push(len, constructor))
    }

    pub fn iter(&self) -> StaticRefArrayIter<T> {
        StaticRefArrayIter {
            vec: &self.0,
            idx: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct StaticRefArrayIter<'a, T> {
    vec: &'a Vec<T>,
    idx: usize,
}

impl<T> StaticRefArrayIter<'_, T> {
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: 'static> Iterator for StaticRefArrayIter<'_, T> {
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.vec.len() {
            return None;
        }

        let ret = unsafe { self.vec.get_unchecked(self.idx).static_ref_mut() };
        self.idx += 1;

        Some(ret)
    }
}

/// Safety: Caller holds
pub unsafe fn static_ref<T: ?Sized>(r: &T) -> &'static T {
    mem::transmute(r)
}

/// Safety: Caller holds
pub unsafe fn static_ref_mut<T: ?Sized>(r: &mut T) -> &'static mut T {
    mem::transmute(r)
}

/// Safety: Caller holds
pub unsafe fn make_mut<T: ?Sized>(r: &T) -> &mut T {
    mem::transmute(r)
}

pub unsafe trait StaticRef<T: ?Sized> {
    unsafe fn static_ref(&self) -> &'static T;
    unsafe fn static_ref_mut(&mut self) -> &'static mut T;
}

unsafe impl<'a, T: ?Sized> StaticRef<T> for &'a T {
    unsafe fn static_ref(&self) -> &'static T {
        static_ref(self)
    }

    unsafe fn static_ref_mut(&mut self) -> &'static mut T {
        static_ref_mut(make_mut(self))
    }
}

unsafe impl<'a, T: ?Sized> StaticRef<T> for &'a mut T {
    unsafe fn static_ref(&self) -> &'static T {
        static_ref(self)
    }

    unsafe fn static_ref_mut(&mut self) -> &'static mut T {
        static_ref_mut(self)
    }
}

#[test]
fn test() {
    static_refs! {
        a = 2;
        b = 33;
    };
    fn need_static(_: &'static i32) {}

    need_static(a);
    need_static(b);
}
