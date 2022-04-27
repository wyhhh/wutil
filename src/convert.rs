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

pub struct StaticRefArray<T>(Vec<T>, usize);

impl<T: Default> StaticRefArray<T> {
    pub fn new(len: usize) -> Self {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::default());
        }
        Self(vec, 0)
    }
}

impl<T: 'static> Iterator for &'_ mut StaticRefArray<T> {
    type Item = &'static mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.0.len() {
            return None;
        }

        let ret = unsafe {self.0.get_unchecked(self.1).static_ref_mut()};
        self.1 += 1;

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
    fn static_ref(&self) -> &'static T;
    fn static_ref_mut(&mut self) -> &'static mut T;
}

unsafe impl<'a, T: ?Sized> StaticRef<T> for &'a T {
    fn static_ref(&self) -> &'static T {
        unsafe { static_ref(self) }
    }

    fn static_ref_mut(&mut self) -> &'static mut T {
        unsafe { static_ref_mut(make_mut(self)) }
    }
}

unsafe impl<'a, T: ?Sized> StaticRef<T> for &'a mut T {
    fn static_ref(&self) -> &'static T {
        unsafe { static_ref(self) }
    }

    fn static_ref_mut(&mut self) -> &'static mut T {
        unsafe { static_ref_mut(self) }
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
