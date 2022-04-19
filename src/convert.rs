use std::mem;

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs {
	($($id:ident = $expr:expr;)+) => {
		$(
			let $id = $expr;
			let $id = unsafe{ $crate::convert::static_ref(&$id) };
		)+
	}
}

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs_mut {
	($($id:ident = $expr:expr;)+) => {
		$(
			let mut $id = $expr;
			let $id = unsafe{ $crate::convert::static_ref_mut(&mut $id) };
		)+
	}
}


/// Safety: Caller holds
pub unsafe fn static_ref<T>(r:&T) -> &'static T {
	mem::transmute(r)
}

/// Safety: Caller holds
pub unsafe fn static_ref_mut<T>(r:&mut T) -> &'static mut T {
	mem::transmute(r)
}

/// Safety: Caller holds
pub unsafe fn make_mut<T>(r:&T) -> &mut T {
	mem::transmute(r)
}

#[test]
fn test() {
	static_refs!{
		a = 2;
		b = 33;
	};
	fn need_static(_:&'static i32) {}

	need_static(a);
	need_static(b);
}