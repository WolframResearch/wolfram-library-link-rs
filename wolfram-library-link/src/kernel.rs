//! Construct and manipulate native Wolfram Kernel expressions.

mod expr_types;
mod predicates;
mod sys;

use std::{ffi::c_void, mem::ManuallyDrop};

use crate::sys::{mint, mreal};

use self::sys::Flags_Expression_UnsignedInteger16;

pub use self::expr_types::{
    Expr, MIntExpr, MRealExpr, NormalExpr, StringExpr, SymbolExpr, UncountedExpr,
};


/// A partially initialized expression value.
///
/// `Uninit` is different from [`std::mem::MaybeUninit`] in that it's meant to be used
/// for values which are *partially* initialized, but which could lead to undefined
/// behavior if the wrong methods were called, or if they were to be dropped.
///
/// Hence, `Uninit` offers a [`assume_init()`][Uninit::assume_init] method.
/// [`std::mem::MaybeUninit`] is a binary condition -- either the underlying value is or
/// is not initialized.
pub struct Uninit<T>(ManuallyDrop<T>);

//======================================
// Impl Expr
//======================================

impl Expr {
    /// Construct a machine-sized Integer expression.
    ///
    /// # Examples
    ///
    /// Construct the expression `42`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::Expr;
    ///
    /// let e = Expr::mint(42);
    /// ```
    pub fn mint(value: mint) -> Expr {
        MIntExpr::new(value).into_expr()
    }

    /// Construct a machine-sized Real expression.
    ///
    /// # Examples
    ///
    /// Construct the expression `1.23`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::Expr;
    ///
    /// let e = Expr::mreal(1.23);
    /// ```
    pub fn mreal(value: mreal) -> Expr {
        MRealExpr::new(value).into_expr()
    }

    /// Construct a String expression.
    ///
    /// # Examples
    ///
    /// Construct the expression `"Hello, Wolfram!"`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::Expr;
    ///
    /// let e = Expr::string("Hello, Wolfram!");
    /// ```
    pub fn string(string: &str) -> Expr {
        StringExpr::new(string).into_expr()
    }

    /// Construct a Symbol expression.
    ///
    /// # Examples
    ///
    /// Construct the expression `` System`Now ``:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::Expr;
    ///
    /// let e = Expr::symbol("System`Now");
    /// ```
    pub fn symbol(symbol: &str) -> Expr {
        // FIXME: Validate that `symbol` is a valid symbol name or fully
        //        qualified symbol.
        SymbolExpr::lookup(symbol).into_expr()
    }

    /// Construct a new `{...}` expression from an array of expressions.
    ///
    /// # Examples
    ///
    /// Construct the expression `{1, 2, 3}`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::Expr;
    ///
    /// let list = Expr::list_from_array([
    ///     Expr::mint(1),
    ///     Expr::mint(2),
    ///     Expr::mint(3)
    /// ]);
    /// ```
    pub fn list_from_array<const N: usize>(array: [Expr; N]) -> Expr {
        NormalExpr::list_from_array(array).into_expr()
    }

    /// Get the expression flags.
    fn flags(&self) -> u16 {
        unsafe { Flags_Expression_UnsignedInteger16(self.to_c_expr()) }
    }
}

//======================================
// Machine Integer Expressions
//======================================

impl MIntExpr {
    /// Construct a new machine integer expression.
    pub fn new(value: mint) -> MIntExpr {
        let signed = true;

        // Sanity check that `mint` can be validly `as`-casted to i64.
        // This would only not be true if/when future computers have a mint size
        // larger than 8 bytes (i.e. where mint is an alias for i128).
        const _: () = assert!(std::mem::size_of::<mint>() <= 8);

        let value = value as i64;

        const BIT_SIZE: u32 = 8 * std::mem::size_of::<mint>() as u32;

        let expr = unsafe { sys::CreateMIntegerExpr(value, BIT_SIZE, signed) };

        unsafe {
            let expr = Expr::from_result(expr);
            MIntExpr::unchecked_from_expr(expr)
        }
    }
}

//======================================
// Machine Real Expressions
//======================================

impl MRealExpr {
    /// Construct a new machine integer expression.
    ///
    /// # Panics
    ///
    /// Panics if `value` is NaN or infinity.
    pub fn new(value: mreal) -> MRealExpr {
        assert!(!value.is_nan() && !value.is_infinite());

        unsafe { MRealExpr::unchecked_new(value) }
    }

    /// Construct a new machine integer expression, without checking for illegal
    /// floating point values.
    ///
    /// # Safety
    ///
    /// `value` must not be a NaN or floating point infinity value.
    pub unsafe fn unchecked_new(mut value: mreal) -> MRealExpr {
        const _: () = assert!(std::mem::size_of::<mreal>() <= 8);

        const BIT_SIZE: u32 = 8 * std::mem::size_of::<mreal>() as u32;

        let value_ptr: *mut f64 = &mut value;
        let value_ptr = value_ptr as *mut c_void;

        let expr = unsafe { sys::CreateMRealExpr(value_ptr, BIT_SIZE) };

        unsafe {
            let expr = Expr::from_result(expr);
            MRealExpr::unchecked_from_expr(expr)
        }
    }
}

//======================================
// Normal Expressions
//======================================

impl NormalExpr {
    /// Construct a new expression with the specified head and length.
    ///
    /// The elements of the resulting expression must be initialized,
    /// using [`write_elem()`][Uninit::<NormalExpr>::write_elem].
    ///
    /// # Examples
    ///
    /// Construct the expression `{1, 2, 3}`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::{Expr, SymbolExpr, NormalExpr};
    ///
    /// let mut expr = NormalExpr::headed(SymbolExpr::lookup("System`List").into(), 3);
    ///
    /// let list: NormalExpr = unsafe {
    ///     expr.write_elem(1, &Expr::mint(1));
    ///     expr.write_elem(2, &Expr::mint(2));
    ///     expr.write_elem(3, &Expr::mint(3));
    ///
    ///     expr.assume_init()
    /// };
    /// ```
    pub fn headed(head: &Expr, len: usize) -> Uninit<NormalExpr> {
        let len = mint::try_from(len).expect("Normal expr length usize overflows mint");

        let normal = unsafe { sys::CreateHeaded_IE_E(len, head.to_c_expr()) };

        // FIXME: `normal` is not fully initialized at this point, so we
        //        shouldn't have a program point where an Expr gets constructed
        //        from it.
        let normal = unsafe { Expr::from_result(normal) };
        let normal = unsafe { NormalExpr::unchecked_from_expr(normal) };

        Uninit(ManuallyDrop::new(normal))
    }

    /// Construct a new `{...}` expression from an array of expressions.
    ///
    /// # Examples
    ///
    /// Construct the expression `{1, 2, 3}`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::{Expr, NormalExpr};
    ///
    /// let list = NormalExpr::list_from_array([
    ///     Expr::mint(1),
    ///     Expr::mint(2),
    ///     Expr::mint(3)
    /// ]);
    /// ```
    pub fn list_from_array<const N: usize>(array: [Expr; N]) -> NormalExpr {
        let mut list = NormalExpr::headed(&SymbolExpr::lookup("System`List").into(), N);

        for (index_0, elem) in array.iter().enumerate() {
            let index_1 = index_0 + 1;
            unsafe { list.write_elem(index_1, elem) }
        }

        unsafe { list.assume_init() }
    }

    /// Construct a normal expression with the given head and arguments.
    ///
    /// # Examples
    ///
    /// Construct the expression `Plus[2, 2]`:
    ///
    /// ```no_run
    /// use wolfram_library_link::kernel::{Expr, NormalExpr};
    /// ```
    pub fn from_slice(head: &Expr, args: &[Expr]) -> NormalExpr {
        let mut normal = NormalExpr::headed(head, args.len());

        for (index_0, elem) in args.into_iter().enumerate() {
            let index_1 = index_0 + 1;
            unsafe { normal.write_elem(index_1, elem) }
        }

        unsafe { normal.assume_init() }
    }
}

impl Uninit<NormalExpr> {
    /// # Safety
    ///
    /// * The current expr must be a Normal expression.
    /// * `index` must not be 0
    /// * `index` can only point to an element which has not been initialized yet.
    pub unsafe fn write_elem(&mut self, index: usize, value: &Expr) {
        let Uninit(self_) = self;

        let self_: &mut NormalExpr = &mut *self_;

        let index = mint::try_from(index).expect("usize index overflows mint");

        sys::SetElement_EIE_E(self_.as_expr().to_c_expr(), index, value.to_c_expr())
    }

    /// Assume that all elements of [`NormalExpr`] have been fully initialized.
    ///
    /// # Safety
    ///
    /// This function must only be called once all elements of this [`NormalExpr`]
    /// have been initialized. It is undefined behavior to construct a
    /// [`NormalExpr`] without first initializing all elements.
    pub unsafe fn assume_init(self) -> NormalExpr {
        let Uninit(expr) = self;

        ManuallyDrop::into_inner(expr)
    }
}

//======================================
// Symbol Expressions
//======================================

impl SymbolExpr {
    /// Lookup or create a symbol expression from a fully qualified symbol or
    /// symbol name.
    ///
    /// FIXME: This function currently does minimal validation that `string` is
    ///        a valid symbol name or fully qualified symbol.
    pub fn lookup(string: &str) -> SymbolExpr {
        let string = StringExpr::new(string);

        unsafe {
            let expr = sys::LookupSymbol_E_E(string.as_expr().to_c_expr());

            let symbol = Expr::from_result(expr);

            SymbolExpr::unchecked_from_expr(symbol)
        }
    }

    /// Set this symbol to the specified value.
    ///
    /// This does NOT evaluate `value` before doing the assignment.
    ///
    /// This is conceptually similar to evaluating
    /// `Set[self, Unevaluated[value]]`.
    pub fn set_to(&self, value: &Expr) -> Expr {
        let expr = unsafe {
            sys::SetSymbol_E_E_E(self.as_expr().to_c_expr(), value.to_c_expr())
        };

        unsafe { Expr::from_result(expr) }
    }

    /// Set downcode used when evaluating `symbol[...]`.
    ///
    /// # Example
    ///
    /// ```
    /// use wolfram_library_link::kernel::{SymbolExpr, Expr, UncountedExpr};
    ///
    /// extern "C" fn my_custom_downcode(expr: UncountedExpr) -> Expr {
    ///     let _normal = NormalExpr::try_from_expr_ref(expr.as_expr()).unwrap();
    ///
    ///     Expr::string("foo")
    /// }
    ///
    /// let symbol = SymbolExpr::lookup("Global`CustomDownCode");
    ///
    /// symbol.set_downcode(Some(eval_downcode))
    /// ```
    ///
    /// Now evaluating `CustomDownCode[arg1, arg2, ...]` will invoke the custom
    /// downcode function `my_custom_downcode`.
    pub fn set_downcode(&self, downcode: Option<extern "C" fn(UncountedExpr) -> Expr>) {
        let downcode = match downcode {
            Some(downcode) => downcode as *mut c_void,
            None => std::ptr::null_mut(),
        };

        unsafe { sys::SetSymbolDownCode(self.as_expr().to_c_expr(), downcode) }
    }
}

//======================================
// String Expressions
//======================================

impl StringExpr {
    /// Construct a new String expression from a UTF-8 encoded string.
    pub fn new(string: &str) -> StringExpr {
        let len = string.len() as mint;
        let string: *const u8 = string.as_ptr();

        let expr = unsafe { sys::UTF8BytesToStringExpression(string, len) };

        unsafe {
            let expr = Expr::from_result(expr);
            StringExpr::unchecked_from_expr(expr)
        }
    }
}

//======================================
// Functions
//======================================

/// Evaluate `Print[e]`
#[allow(non_snake_case)]
pub fn Print(e: &Expr) {
    unsafe {
        sys::Print_E_I(e.to_c_expr());
    }
}

/// Evaluate the given expression, returning the resulting expression.
///
/// # Examples
///
/// ```no_run
/// use wolfram_library_link::kernel::{self, Expr, NormalExpr, MIntExpr};
///
/// let result = kernel::eval(
///     &NormalExpr::from_slice(&Expr::symbol("System`Plus"), &[
///         Expr::mint(2),
///         Expr::mint(2),
///     ])
///     .into(),
/// );
///
/// let result = MIntExpr::try_from_expr(result).unwrap();
/// ```
pub fn eval(e: &Expr) -> Expr {
    unsafe {
        let result = sys::Evaluate_E_E(e.to_c_expr());

        Expr::from_result(result)
    }
}
