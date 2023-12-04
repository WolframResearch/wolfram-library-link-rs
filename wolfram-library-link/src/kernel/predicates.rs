#![allow(non_snake_case)]

use crate::kernel::Expr;

/// A type value only uses the first 4 bits.
const TYPE_MASK: u16 = 0b1111;

mod expr_type {
    pub const MACHINE_INTEGER: u16 = 0;
    pub const MACHINE_REAL: u16 = 2;

    pub const NORMAL: u16 = 6;
    pub const SYMBOL: u16 = 7;
    pub const STRING: u16 = 8;
}

impl Expr {
    #[inline]
    pub(crate) fn type_flags(&self) -> u16 {
        self.flags() & TYPE_MASK
    }
}

pub(crate) fn MIntegerQ(expr: &Expr) -> bool {
    expr.type_flags() == expr_type::MACHINE_INTEGER
}

pub(crate) fn MRealQ(expr: &Expr) -> bool {
    expr.type_flags() == expr_type::MACHINE_REAL
}

pub(crate) fn NormalQ(expr: &Expr) -> bool {
    expr.type_flags() == expr_type::NORMAL
}

pub(crate) fn SymbolQ(expr: &Expr) -> bool {
    expr.type_flags() == expr_type::SYMBOL
}

pub(crate) fn StringQ(expr: &Expr) -> bool {
    expr.type_flags() == expr_type::STRING
}
