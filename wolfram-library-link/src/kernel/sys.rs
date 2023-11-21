#![allow(non_camel_case_types)]

use std::{
    ffi::{c_uchar, c_void},
    marker::{PhantomData, PhantomPinned},
};

use wolfram_library_link_sys::mint;

#[repr(C)]
pub struct expr_struct {
    _data: [u8; 0],
    // Prevent this type from being auto Send, Sync, or Pin.
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub(crate) type expr = *mut expr_struct;

//
// Type declarations available from compiler type system setup code for
// Native`PrimitiveFunction values, in:
//
// $InstallationDirectory/SystemFiles/Components/Compile/TypeSystem/Declarations/
//

#[rustfmt::skip]
extern "C" {
    pub(crate) fn Print_E_I(arg: expr) -> mint;
    pub(crate) fn CreateMIntegerExpr(i: i64, size: u32, signedQ: bool) -> expr;
    pub(crate) fn CreateMRealExpr(src: *mut c_void, size: u32) -> expr;
    pub(crate) fn LookupSymbol_E_E(arg: expr) -> expr;
    pub(crate) fn SetSymbol_E_E_E(arg1: expr, arg2: expr) -> expr;
    pub(crate) fn CreateHeaded_IE_E(len: mint, head: expr) -> expr;
    pub(crate) fn SetElement_EIE_E(base_arg: expr, pos: mint, elem_arg: expr);
    pub(crate) fn LoadEFAIL() -> expr;
    pub(crate) fn LoadENULL() -> expr;
    pub(crate) fn Runtime_DecrementRefCount(e: *mut expr) -> mint;
    pub(crate) fn Flags_Expression_UnsignedInteger16(arg: expr) -> u16;
    pub(crate) fn UTF8BytesToStringExpression(data: *const c_uchar, len: mint) -> expr;
    pub(crate) fn Evaluate_E_E(arg: expr) -> expr;
}
