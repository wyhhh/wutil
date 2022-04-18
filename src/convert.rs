use std::mem;

/// Safety: Caller holds
pub unsafe fn static_ref<T>(r:&T) -> &'static T {
	mem::transmute(r)
}

/// Safety: Caller holds
pub unsafe fn static_ref_mut<T>(r:&mut T) -> &'static mut T {
	mem::transmute(r)
}