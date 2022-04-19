use std::mem;

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs {
	($($id:ident = $ref:expr;)+) => {
		$(let $id = unsafe{ static_ref($ref) };)+
	}
}

/// Caller makes sure the safety
#[macro_export]
macro_rules! static_refs_mut {
	($($id:ident = $ref:expr;)+) => {
		$(let $id = unsafe{ static_ref_mut($ref) };)+
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

#[test]
fn test() {
	let n = 3;
	static_refs!{
		a = &2;
		b = &n;
	};
	fn need_static(_:&'static i32) {}

	need_static(a);
	need_static(b);
}