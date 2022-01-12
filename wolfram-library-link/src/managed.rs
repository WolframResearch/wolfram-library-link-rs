//! Managed expressions.
//!
//! Managed expressions are Wolfram Language expressions created using
//! [`CreateManagedLibraryExpression`][ref/CreateManagedLibraryExpression]<sub>WL</sub>,
//! which are associated with a unique [`Id`] number that is shared with a loaded library.
//!
//! Using [`register_library_expression_manager()`], a library can register a callback
//! function, which will recieve a [`ManagedExpressionEvent`] each time a new managed
//! expression is created or deallocated.
//!
//! The managed expression [`Create(Id)`][ManagedExpressionEvent::Create] event is
//! typically handled by the library to create an instance of some library data type that
//! is associated with the managed expression. When the managed expression is finally
//! deallocated, a [`Drop(Id)`][ManagedExpressionEvent::Drop] event is generated, and
//! the library knows it is safe to free the associated data object.
//!
//! In this way, managed expressions allow memory-management of Rust objects to be
//! performed indirectly based on the lifetime of a Wolfram Language expression.
//!
//  TODO: Expand and polish this section: # Alternatives
//
//  * Canonical WL expression representation
//  * MyStruct[<some raw pointer value>] and manual memory management from WL
//
//  Managed expressions are a way for objects that cannot be easily or efficiently
//  represented as a Wolfram Language expression to still be associated with the lifetime
//  of a Wolfram Language expression.
//
//  If an object can be represented as a Wolfram expression, then that is the most
//  straightforward thing to do. the
//  simplest possible [[ToExpr, FromExpr]].
//
//  The simplest alternative to managed expressions to simply convert the
//
//! # Related links
//!
//! * [Managed Library Expressions] section of the LibraryLink documentation.
//!
//! [Managed Library Expressions]: https://reference.wolfram.com/language/LibraryLink/tutorial/InteractionWithWolframLanguage.html#353220453
//! [ref/CreateManagedLibraryExpression]: https://reference.wolfram.com/language/ref/CreateManagedLibraryExpression.html

use std::{ffi::CString, sync::Mutex};

use once_cell::sync::Lazy;

use crate::{rtl, sys};

/// Lifecycle events triggered by the creation and deallocation of managed expressions.
pub enum ManagedExpressionEvent {
    /// Instruction that the library should create a new instance of a managed expression
    /// with the specified [`Id`].
    ///
    /// This event occurs when
    /// [CreateManagedLibraryExpression][ref/CreateManagedLibraryExpression] is called.
    ///
    /// [ref/CreateManagedLibraryExpression]: https://reference.wolfram.com/language/ref/CreateManagedLibraryExpression.html
    Create(Id),
    /// Instruction that the library should drop any data associated with the managed
    /// expression identified by this [`Id`].
    ///
    /// This event occurs when the managed expression is no longer used by the Wolfram
    /// Language.
    Drop(Id),
}

impl ManagedExpressionEvent {
    /// Get the managed expression [`Id`] targeted by this action.
    pub fn id(&self) -> Id {
        match *self {
            ManagedExpressionEvent::Create(id) => id,
            ManagedExpressionEvent::Drop(id) => id,
        }
    }
}

/// Unique identifier associated with an instance of a managed library expression.
pub type Id = u32;

/// Register a new callback function for handling managed expression events.
pub fn register_library_expression_manager(
    name: &str,
    manage_instance: fn(ManagedExpressionEvent),
) {
    register_using_next_slot(name, manage_instance)
}

//======================================
// C wrapper functions
//======================================

/// # Implementation note on the reason for this static / "slot" system.
///
/// Having this static is not a direct requirement of the library expression
/// C API, however it is necessary as a workaround for the problem described below.
///
/// `registerLibraryExpressionManager()` expects a callback function of the type:
///
///     unsafe extern "C"(WolframLibraryData, mbool, mint)
///
/// however, for the purpose of providing a more ergonomic and safe wrapper to the user,
/// we want the user to be able to pass `register_library_expression_manager()` a callback
/// function with the type:
///
///     fn(ManagedExpressionAction)
///
/// This specific problem is an instance of the more general problem of how to expose a
/// user-provided function/closure (non-`extern "C"`) as-if it actually were an
/// `extern "C"` function.
///
/// There are two common ways we could concievably do this:
///
/// 1. Use a macro to generate an `extern "C"` function that calls the user-provided
///    function.
///
/// 2. Use a "trampoline" function (e.g. like async_task_thread_trampoline()) which has
///    the correct `extern "C"` signature, and wraps the user function. This only works
///    if the `extern "C"` function has a parameter that we can control and use to pass
///    in a function pointer to the user-provided function.
///
/// The (1.) strategy is easy to implement, but is undesirable because:
///
///   a) it requires the user to use a macro -- and for subtle reasons (see: this comment)).
///   b) it exposes the underlying `unsafe extern "C" fn(...)` type to the user.
///
/// The (2.) strategy is often a good choice, but cannot be used in this particular
/// case, because their is no way to pass a custom argument to the callback expected by
/// registerLibraryExpressionManager().
///
/// The technique used here is a third strategy:
///
/// 3. Store the user-provided function pointer into a static array, and, instead of
///    having a single `extern "C"` wrapper function, have multiple `extern "C"` wrapper
///    functions, each of which statically access a different index in the static array.
///
///    By using different `extern "C"` functions that access different static data, we
///    can essentially "fake" having an extra function argument that we control.
///
///    This depends on the observation that the callback function pointer is itself a
///    value we control.
///
///    This technique is limited by the fact that the static function pointers must be
///    declared ahead of time (see `def_slot_fn!` below), and so practically there is a
///    somewhat arbitrary limit on how many callbacks can be registered at a time.
///
/// In our case, the *only* data we are able pass through the C API is the static function
/// pointer we are registering; so strategy (3.) is the way to go.
///
/// `SLOTS` has 8 elements, and we define 8 `extern "C" fn slot_<X>(..)` functions that
/// access only the corresponding element in `SLOTS`.
///
/// 8 was picked arbitrarily, on the assumption that 8 different registered types should
/// be sufficient for the vast majority of libraries. Libraries that want to register more
/// than 8 types can use `rtl::registerLibraryExpressionManager` directly as a workaround.
///
/// TODO: Also store the "name" of this manager, and pass it to the user function?
static SLOTS: Lazy<Mutex<[Option<fn(ManagedExpressionEvent)>; 4]>> =
    Lazy::new(|| Mutex::new([None, None, None, None]));

fn register_using_next_slot(name: &str, manage_instance: fn(ManagedExpressionEvent)) {
    let name_cstr = CString::new(name).expect("failed to allocate C string");

    let mut slots = SLOTS.lock().unwrap();

    let available_slot: Option<(usize, &mut Option<_>)> = slots
        .iter_mut()
        .enumerate()
        .filter(|(_, slot)| slot.is_none())
        .next();

    let result = if let Some((index, slot)) = available_slot {
        *slot = Some(manage_instance);
        register_using_slot(name_cstr, index)
    } else {
        // Drop `slots` to avoid poisoning SLOTS when we panic.
        drop(slots);
        panic!("maxiumum number of library expression managers have been registered");
    };

    drop(slots);

    if let Err(()) = result {
        panic!(
            "library expression manager with name '{}' has already been registered",
            name
        );
    }
}

fn register_using_slot(name_cstr: CString, index: usize) -> Result<(), ()> {
    let static_slot_fn: unsafe extern "C" fn(_, _, _) = match index {
        0 => slot_0,
        1 => slot_1,
        2 => slot_2,
        3 => slot_3,
        4 => slot_4,
        5 => slot_5,
        6 => slot_6,
        7 => slot_7,
        8 => slot_8,
        _ => unreachable!(),
    };

    let err_code: i32 = unsafe {
        rtl::registerLibraryExpressionManager(name_cstr.as_ptr(), Some(static_slot_fn))
    };

    if err_code != 0 {
        Err(())
    } else {
        Ok(())
    }
}

//--------------------------
// Static slot_<X> functions
//--------------------------

fn call_using_slot<const SLOT: usize>(mode: sys::mbool, id: sys::mint) {
    let slots = SLOTS.lock().unwrap();

    let user_fn: fn(ManagedExpressionEvent) = match slots[SLOT] {
        Some(func) => func,
        // TODO: Set something like "RustLink`$LibraryLastError" with a descriptive error?
        None => return,
    };

    // Ensure we're not holding a lock on `slots`, to avoid poisoning SLOTS in the case
    // `user_fn` panics.
    drop(slots);

    let id: u32 = match u32::try_from(id) {
        Ok(id) => id,
        // TODO: Set something like "RustLink`$LibraryLastError" with a descriptive error?
        Err(_) => return,
    };

    let action = match mode {
        0 => ManagedExpressionEvent::Create(id),
        1 => ManagedExpressionEvent::Drop(id),
        _ => panic!("unknown managed expression 'mode' value: {}", mode),
    };

    if let Err(_) = crate::catch_panic::call_and_catch_panic(|| user_fn(action)) {
        // Do nothing.
        // TODO: Set something like "RustLink`$LibraryLastError" with this panic?
    }
}

macro_rules! def_slot_fn {
    ($name:ident, $index:literal) => {
        unsafe extern "C" fn $name(
            // Assume this library is already initialized.
            _: sys::WolframLibraryData,
            mode: sys::mbool,
            id: sys::mint,
        ) {
            call_using_slot::<$index>(mode, id)
        }
    };
}

def_slot_fn!(slot_0, 0);
def_slot_fn!(slot_1, 1);
def_slot_fn!(slot_2, 2);
def_slot_fn!(slot_3, 3);
def_slot_fn!(slot_4, 4);
def_slot_fn!(slot_5, 5);
def_slot_fn!(slot_6, 6);
def_slot_fn!(slot_7, 7);
def_slot_fn!(slot_8, 8);
