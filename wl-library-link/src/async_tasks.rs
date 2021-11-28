//! Support for Wolfram Language asynchronous tasks.
//!
//! # Credits
//!
//! The implementations of this module and the associated examples are based on the path
//! laid out by [this StackOverflow answer](https://mathematica.stackexchange.com/a/138433).

use std::{
    ffi::{c_void, CString},
    panic,
};

use static_assertions::assert_not_impl_any;

use crate::{
    sys::{self, mint},
    DataStore,
};


/// Handle to a Wolfram Language [`AsynchronousTaskObject`](https://reference.wolfram.com/language/ref/AsynchronousTaskObject.html)
/// instance.
///
/// Use [`spawn_async_task_with_thread()`] to spawn a new asynchronous task.
#[derive(Debug)]
pub struct AsyncTaskObject(sys::mint);

// TODO: Determine if it would be safe for this type to implement Copy/Clone.
assert_not_impl_any!(AsyncTaskObject: Copy, Clone);

/// See [`spawn_async_task_with_thread()`].
///
/// This trait is a workaround used to implement named type aliases with trait bounds.
pub trait AsyncTask: FnMut(AsyncTaskObject) + Send + 'static + panic::UnwindSafe {}

/// Blanket impl of [`AsyncTask`] for suitable closures.
impl<T: FnMut(AsyncTaskObject) + Send + 'static + panic::UnwindSafe> AsyncTask for T {}

//======================================
// Impls
//======================================

impl AsyncTaskObject {
    /// Returns the numeric ID which identifies this async object.
    pub fn id(&self) -> sys::mint {
        let AsyncTaskObject(id) = *self;
        id
    }

    /// Returns whether this async task is still alive.
    ///
    /// *LibraryLink C Function:* [`asynchronousTaskAliveQ`][sys::st_WolframIOLibrary_Functions::asynchronousTaskAliveQ].
    pub fn is_alive(&self) -> bool {
        let io_funcs = unsafe { *crate::get_library_data().ioLibraryFunctions };

        let async_task_is_alive: unsafe extern "C" fn(sys::mint) -> std::os::raw::c_int =
            io_funcs
                .asynchronousTaskAliveQ
                .expect("asynchronousTaskAliveQ callback is NULL");

        let is_alive: i32 = unsafe { async_task_is_alive(self.id()) };

        is_alive != 0
    }

    /// Returns whether this async task has been started.
    ///
    /// *LibraryLink C Function:* [`asynchronousTaskStartedQ`][sys::st_WolframIOLibrary_Functions::asynchronousTaskStartedQ].
    pub fn is_started(&self) -> bool {
        let io_funcs = unsafe { *crate::get_library_data().ioLibraryFunctions };

        let async_task_is_started: unsafe extern "C" fn(
            sys::mint,
        ) -> std::os::raw::c_int = io_funcs
            .asynchronousTaskStartedQ
            .expect("asynchronousTaskStartedQ callback is NULL");

        let is_started: i32 = unsafe { async_task_is_started(self.id()) };

        is_started != 0
    }

    /// Raise a new named asynchronous event associated with the current async task.
    ///
    /// # Example
    ///
    /// Raise a new asynchronous event with no associated data:
    ///
    /// This will cause the Wolfram Language event handler associated with this task to
    /// be run.
    ///
    /// *LibraryLink C Function:* [`raiseAsyncEvent`][sys::st_WolframIOLibrary_Functions::raiseAsyncEvent].
    ///
    /// ```no_run
    /// use wl_library_link::{AsyncTaskObject, DataStore};
    ///
    /// let task_object: AsyncTaskObject = todo!();
    ///
    /// task_object.raise_async_event("change", DataStore::new());
    /// ```
    pub fn raise_async_event(&self, name: &str, data: DataStore) {
        use std::os::raw::c_char;

        let AsyncTaskObject(id) = *self;

        let name = CString::new(name)
            .expect("unable to convert raised async event name to CString");

        let raise_async_event: unsafe extern "C" fn(mint, *mut c_char, sys::DataStore) = {
            let io_funcs = unsafe { *crate::get_library_data().ioLibraryFunctions };

            io_funcs
                .raiseAsyncEvent
                .expect("raiseAsyncEvent callback is NULL")
        };

        unsafe {
            // raise_async_event(id, name.as_ptr() as *mut c_char, data.into_ptr());
            raise_async_event(id, name.into_raw(), data.into_ptr());
        }
    }
}

/// Spawn a new Wolfram Language asynchronous task.
pub fn spawn_async_task_with_thread<F: AsyncTask>(task: F) -> AsyncTaskObject {
    let create_async_task_with_thread: unsafe extern "C" fn(
        Option<unsafe extern "C" fn(i64, *mut c_void)>,
        *mut c_void,
    ) -> sys::mint = unsafe {
        (*crate::get_library_data().ioLibraryFunctions)
            .createAsynchronousTaskWithThread
            .expect("createAsynchronousTaskWithThread callback is NULL")
    };

    // FIXME: This box is being leaked. Where is an appropriate place to drop it?
    let boxed_closure = Box::into_raw(Box::new(task));

    // Spawn a background thread using the user closure.
    let task_id: sys::mint = unsafe {
        create_async_task_with_thread(
            Some(async_task_thread_trampoline::<F>),
            boxed_closure as *mut c_void,
        )
    };

    AsyncTaskObject(task_id)
}

unsafe extern "C" fn async_task_thread_trampoline<F: AsyncTask>(
    async_object_id: sys::mint,
    boxed_closure: *mut c_void,
) {
    let boxed_closure: &mut F = &mut *(boxed_closure as *mut F);

    // static_assertions::assert_impl_all!(F: panic::UnwindSafe);

    // Catch any panics which occur.
    //
    // Use AssertUnwindSafe because:
    //   1) `F` is already required to implement UnwindSafe by the definition of AsyncTask.
    //   2) We don't introduce any new potential unwind safety with our minimal closure
    //      here.
    match panic::catch_unwind(panic::AssertUnwindSafe(|| {
        boxed_closure(AsyncTaskObject(async_object_id))
    })) {
        Ok(()) => (),
        Err(_) => (),
    }
}
