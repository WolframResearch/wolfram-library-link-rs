use std::{marker::PhantomData, mem::ManuallyDrop};

use crate::kernel::{predicates, sys};

//==========================================================
// Fundamental expression wrapper types
//==========================================================

/// Wolfram Kernel expr.
///
/// Wolfram Kernel expressions are immutable.
#[derive(Debug)]
#[repr(transparent)]
pub struct Expr(sys::expr);

//======================================
// UncountedExpr
//======================================

/// Represents an uncounted copy of an expression, which is tied to the lifetime
/// of a counted [`Expr`].
///
/// This type is used in the signature of the callback function pointer
/// accepted by [`SymbolExpr::set_downcode()`].
///
/// # Implementation
///
/// The use of [`ManuallyDrop`] prevents the underlying [`Expr`] from being dropped (and
/// therefore getting RefDecr'd) when the `UncountedExpr` is dropped. This maintains the
/// invariant that `UncountedExpr` doesn't own a +1 count of the underlying expression.
///
/// The use of the [`PhantomData`] lifetime prevents an `UncountedExpr` from living longer
/// than the [`Expr`] it's borrowed from.
//
// TODO: Could also be called `ExprRef` (though this might be confused with the `*Ref`
//       naming convention used by SymbolRef/NormalRef/StringRef/BigIntRef/etc.).
#[repr(transparent)]
pub struct UncountedExpr<'e> {
    /// This type MUST NOT provide users of this API with owned access to this
    /// value.
    expr: ManuallyDrop<Expr>,
    phantom: PhantomData<&'e Expr>,
}

impl<'e> UncountedExpr<'e> {
    pub fn as_expr(&self) -> &Expr {
        let UncountedExpr { expr, phantom: _ } = self;

        &*expr
    }
}

//==========================================================
// Typed Expression wrapper types
//==========================================================

/// Generate all the common methods for an expression wrapper type.
macro_rules! expr_wrapper {
    (
        $(#[$outer:meta])* struct $name:ident,
        $predicate:ident
    ) => {
        // TODO: Provide a better Debug implementation for these types
        #[derive(Debug)]
        $(#[$outer])*
        #[derive(ref_cast::RefCastCustom)]
        #[repr(transparent)]
        pub struct $name(Expr);

        /// General expression wrapper type methods.
        impl $name {
            /// Attempt to convert `expr` into `Self`.
            #[inline(always)]
            pub fn try_from_expr(expr: Expr) -> Option<Self> {
                if predicates::$predicate(&expr) {
                    Some(unsafe { $name::unchecked_from_expr(expr) })
                } else {
                    None
                }
            }

            /// Attempt to convert `expr` into `&Self`.
            #[inline(always)]
            pub fn try_from_expr_ref(expr: &Expr) -> Option<&Self> {
                if predicates::$predicate(&expr) {
                    Some(unsafe { $name::unchecked_from_expr_ref(expr) })
                } else {
                    None
                }
            }

            /// Private.
            ///
            /// Unsafely interpret an [`Expr`] as an instance of this expression wrapper
            /// type.
            ///
            /// Strongly prefer using [`try_from_expr()`][Self::try_from_expr()] instead
            /// of this method.
            ///
            /// # Safety
            ///
            /// `expr` must satisfy the predicate associated with this expression wrapper
            /// type.
            #[inline]
            pub(crate) unsafe fn unchecked_from_expr(expr: Expr) -> Self {
                debug_assert!(predicates::$predicate(&expr));

                $name(expr)
            }

            /// # Safety
            ///
            /// `expr` must satisfy the predicate associated with this expression wrapper
            /// type.
            #[inline]
            #[ref_cast::ref_cast_custom]
            pub(crate) unsafe fn unchecked_from_expr_ref(expr: &Expr) -> &Self;

            // #[allow(dead_code)]
            // pub(in crate::expr) unsafe fn unchecked_from_expr_ref_mut(expr: &mut Expr) -> &mut Self {
            //     debug_assert!(predicates::$predicate(expr));

            //     expr_ref_cast_mut::<Expr, Self>(expr)
            // }

            /// Get a reference to the underlying [`Expr`] this type wraps.
            pub fn as_expr(&self) -> &Expr {
                let $name(e) = self;
                e
            }

            #[allow(dead_code)]
            pub(crate) fn as_expr_mut(&mut self) -> &mut Expr {
                let $name(e) = self;
                e
            }

            /// Get the underlying [`Expr`] this type wraps.
            pub fn into_expr(self) -> Expr {
                let $name(e) = self;
                e
            }
        }

        impl AsRef<Expr> for $name {
            fn as_ref(&self) -> &Expr {
                $name::as_expr(self)
            }
        }

        impl From<$name> for Expr {
            fn from(e: $name) -> Expr {
                $name::into_expr(e)
            }
        }
    };
}

expr_wrapper![
    /// Machine integer expression wrapper.
    ///
    /// This type represents an expression which has been validated to be a
    /// machine integer.
    ///
    /// Use [`MIntExpr::as_expr()`] and [`MIntExpr::into_expr()`] to access the underlying
    /// [`Expr`].
    struct MIntExpr,
    MIntegerQ
];

expr_wrapper![
    /// Machine real expression wrapper.
    ///
    /// This type represents an expression which has been validated to be a
    /// machine real.
    ///
    /// Use [`MRealExpr::as_expr()`] and [`MRealExpr::into_expr()`] to access the underlying
    /// [`Expr`].
    struct MRealExpr,
    MRealQ
];

expr_wrapper![
    /// Normal expression wrapper.
    ///
    /// This type represents an expression which has been validated to be a Normal.
    ///
    /// Use [`NormalExpr::as_expr()`] and [`NormalExpr::into_expr()`] to access the underlying
    /// [`Expr`].
    struct NormalExpr,
    NormalQ
];

expr_wrapper![
    /// Symbol expression wrapper.
    ///
    /// This type represents an expression which has been validated to be a Symbol.
    ///
    /// Use [`SymbolExpr::as_expr()`] and [`SymbolExpr::into_expr()`] to access the underlying
    /// [`Expr`].
    struct SymbolExpr,
    SymbolQ
];

expr_wrapper![
    /// String expression wrapper.
    ///
    /// This type represents an expression which has been validated to be a String.
    ///
    /// The `E` prefix distinguishes this type from the standard library [`String`] type.
    ///
    /// Use [`StringExpr::as_expr()`] and [`StringExpr::into_expr()`] to access the underlying
    /// [`Expr`].
    struct StringExpr,
    StringQ
];

//======================================
// Impl Expr
//======================================

impl Expr {
    /// # Panics
    ///
    /// Panics if `expr` is not a success result.
    pub(crate) unsafe fn from_result(expr: sys::expr) -> Expr {
        Expr::try_from_result(expr).expect("Got EFAIL or ENULL")
    }

    pub(crate) unsafe fn try_from_result(expr: sys::expr) -> Option<Expr> {
        unsafe {
            if expr == sys::LoadEFAIL() || expr == sys::LoadENULL() {
                return None;
            }
        }

        Some(Expr(expr))
    }

    /// Take ownership of `self` and return the underyling expression pointer.
    #[allow(dead_code)]
    pub(crate) unsafe fn into_c_expr(self) -> sys::expr {
        let Expr(e) = self;

        // Don't drop `self` if it's going to be used as a raw `expr`.
        // Note: We have to explicitly forget `self` because the move of `e` above is
        //       actually a copy; `self` is still valid even though it appears we moved
        //       ownership out of it.
        std::mem::forget(self);

        e
    }

    /// Access the C [`expr`][sys::expr] type wrapped by this `Expr`.
    ///
    /// This function returns what is conceptually a *borrowed* `expr`.
    ///
    /// If you need an owned, counted copy of the C expr, use [`Expr::into_c_expr()`].
    #[inline(always)]
    pub(crate) unsafe fn to_c_expr(&self) -> sys::expr {
        let Expr(inner) = *self;

        inner
    }
}

//======================================
// Drop Impls
//======================================

impl Drop for Expr {
    fn drop(&mut self) {
        let Expr(expr) = self;

        let expr: &mut sys::expr = expr;

        unsafe {
            sys::Runtime_DecrementRefCount(expr);
        }
    }
}
