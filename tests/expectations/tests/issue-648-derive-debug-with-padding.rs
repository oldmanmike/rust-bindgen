/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]



/// We emit a `[u8; 63usize]` padding field for this struct, which cannot derive
/// Debug/Hash because 63 is over the hard coded limit. (Yes, this struct doesn't end
/// up with the reight alignment, we're waiting on `#[repr(align="N")]` to land
/// in rustc).
#[repr(C)]
#[derive(Copy)]
pub struct NoDebug {
    pub c: ::std::os::raw::c_char,
    pub __bindgen_padding_0: [u8; 63usize],
}
#[test]
fn bindgen_test_layout_NoDebug() {
    assert_eq!(
        ::std::mem::size_of::<NoDebug>(),
        64usize,
        concat!("Size of: ", stringify!(NoDebug))
    );
    assert_eq!(
        unsafe { &(*(0 as *const NoDebug)).c as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(NoDebug),
            "::",
            stringify!(c)
        )
    );
}
impl Clone for NoDebug {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for NoDebug {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NoDebug {
    fn eq(&self, other: &NoDebug) -> bool {
        self.c == other.c
    }
}
/// This should derive Debug/Hash/PartialEq/Eq because the padding size is less than the max derive
/// Debug/Hash/PartialEq/Eq impl for arrays. However, we conservatively don't derive Debug/Hash because
/// we determine Debug derive-ability before we compute padding, which happens at
/// codegen. (Again, we expect to get the alignment wrong for similar reasons.)
#[repr(C)]
#[derive(Copy)]
pub struct ShouldDeriveDebugButDoesNot {
    pub c: [::std::os::raw::c_char; 32usize],
    pub d: ::std::os::raw::c_char,
    pub __bindgen_padding_0: [u8; 31usize],
}
#[test]
fn bindgen_test_layout_ShouldDeriveDebugButDoesNot() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveDebugButDoesNot>(),
        64usize,
        concat!("Size of: ", stringify!(ShouldDeriveDebugButDoesNot))
    );
    assert_eq!(
        unsafe { &(*(0 as *const ShouldDeriveDebugButDoesNot)).c as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(ShouldDeriveDebugButDoesNot),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const ShouldDeriveDebugButDoesNot)).d as *const _ as usize },
        32usize,
        concat!(
            "Alignment of field: ",
            stringify!(ShouldDeriveDebugButDoesNot),
            "::",
            stringify!(d)
        )
    );
}
impl Clone for ShouldDeriveDebugButDoesNot {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ShouldDeriveDebugButDoesNot {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ShouldDeriveDebugButDoesNot {
    fn eq(&self, other: &ShouldDeriveDebugButDoesNot) -> bool {
        self.c == other.c && self.d == other.d
    }
}
