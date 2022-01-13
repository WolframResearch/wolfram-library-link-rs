use std::thread;

use once_cell::sync::OnceCell;

use crate::sys::{
    self, mbool, mcomplex, mint, mreal, st_WolframCompileLibrary_Functions,
    st_WolframIOLibrary_Functions, st_WolframImageLibrary_Functions,
    st_WolframNumericArrayLibrary_Functions, st_WolframRawArrayLibrary_Functions,
    st_WolframRuntimeData, st_WolframSparseLibrary_Functions, MArgument, MInputStream,
    MOutputStream, MTensor, WSENV, WSLINK,
};

#[derive(Copy, Clone)]
struct Data {
    /// The `ThreadId` of the Wolfram Kernel's main thread.
    ///
    /// The main evaluation loop of the Wolfram Kernel is largely a single-threaded
    /// program, and it's functions are not all necessarily designed to be used from
    /// multiple threads at once. This value, used in [`assert_main_thread()`], is used to
    /// ensure that the safe API's provided by `wolfram-library-link` are only called from
    /// the main Kernel thread.
    main_thread_id: thread::ThreadId,
    library_data: WolframLibraryData,
}

static LIBRARY_DATA: OnceCell<Data> = OnceCell::new();

/// Initialize static data for the current Wolfram library.
///
/// This function should be called during the execution of the
/// [`WolframLibrary_initialize()` hook][lib-init]
/// provided by this library.
///
/// This function initializes the lazy Wolfram Runtime Library bindings in the
/// [`rtl`][`crate::rtl`] module.
///
/// # Safety
///
/// The following conditions must be met for a call to this function to be valid:
///
/// * `data` must be a valid and fully initialized [`sys::WolframLibraryData`] instance
///   created by the Wolfram Kernel and passed into the current LibraryLink function.
/// * The call to `initialize()` must happen from the main Kernel thread. This is true for
///   all LibraryLink functions called directly by the Kernel.
///
/// # Relation to [`#[init]`][crate::init]
///
/// If the [`#[init]`][crate::init] annotation is used to designate a library
/// initialization function, `initialize()` will be called automatically.
///
/// # Example
///
/// *Note: Prefer to use [`#[init]`][crate::init] to designate an initialization function,
/// instead of manually defining an unsafe initialization function as shown in this
/// example.*
///
/// When a dynamic library is loaded by the Wolfram Language (for example, via
/// [`LibraryFunctionLoad`](https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html)),
/// the system will call the function `WolframLibrary_initialize()` if one is provided by
/// the library. This function is used to initialize callbacks from the library into
/// functions provided by the Wolfram runtime.
///
/// ```
/// use std::os::raw::c_int;
/// use wolfram_library_link::{sys, initialize};
///
/// #[no_mangle]
/// extern "C" fn WolframLibrary_initialize(data: sys::WolframLibraryData) -> c_int {
///     match unsafe { initialize(data) } {
///         Ok(()) => return 0,
///         Err(()) => return 1,
///     }
/// }
/// ```
///
/// [lib-init]: https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#280210622
pub unsafe fn initialize(data: sys::WolframLibraryData) -> Result<(), ()> {
    let library_data = WolframLibraryData::new(data)?;

    let _: Result<(), Data> = LIBRARY_DATA.set(Data {
        main_thread_id: thread::current().id(),
        library_data,
    });

    Ok(())
}

/// Get the [`WolframLibraryData`] instance recorded by the last call to [`initialize()`].
///
/// Prefer to use the lazy function bindings from the [`rtl`][crate::rtl] module instead
/// of accessing the fields of [`WolframLibraryData`] directly.
pub fn get_library_data() -> WolframLibraryData {
    let data: Option<&_> = LIBRARY_DATA.get();

    // TODO: Include a comment here mentioning that the library could/should provide a
    //       WolframLibrary_initialize() function which calls initialize_library_data()?
    data.expect(
        "get_library_data: global Wolfram LIBRARY_DATA static is not initialized.",
    )
    .library_data
}

pub(crate) fn is_main_thread() -> bool {
    let data = LIBRARY_DATA
        .get()
        .expect("global LIBRARY_DATA static is not initialized");

    data.main_thread_id == thread::current().id()
}

/// Assert that the current thread is the main Kernel thread.
///
/// # Panics
///
/// This function will panic if the current thread is not the main Kernel thread.
///
/// Use this function to enforce that callbacks into the Kernel happen from the
/// main thread.
#[track_caller]
pub(crate) fn assert_main_thread() {
    let loc = std::panic::Location::caller();

    assert!(
        is_main_thread(),
        "error: attempted to call back into the Wolfram Kernel from a non-main thread at {}:{}",
        loc.file(),
        loc.line()
    );
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
#[allow(missing_docs)]
pub struct WolframLibraryData {
    pub raw_library_data: sys::WolframLibraryData,

    pub UTF8String_disown: unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_char),

    pub MTensor_new: unsafe extern "C" fn(
        arg1: mint,
        arg2: mint,
        arg3: *const mint,
        arg4: *mut MTensor,
    ) -> ::std::os::raw::c_int,

    pub MTensor_free: unsafe extern "C" fn(arg1: MTensor),

    pub MTensor_clone:
        unsafe extern "C" fn(arg1: MTensor, arg2: *mut MTensor) -> ::std::os::raw::c_int,

    pub MTensor_shareCount: unsafe extern "C" fn(arg1: MTensor) -> mint,

    pub MTensor_disown: unsafe extern "C" fn(arg1: MTensor),

    pub MTensor_disownAll: unsafe extern "C" fn(arg1: MTensor),

    pub MTensor_setInteger: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: mint,
    ) -> ::std::os::raw::c_int,

    pub MTensor_setReal: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: mreal,
    ) -> ::std::os::raw::c_int,

    pub MTensor_setComplex: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: mcomplex,
    ) -> ::std::os::raw::c_int,

    pub MTensor_setMTensor: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: MTensor,
        arg3: *mut mint,
        arg4: mint,
    ) -> ::std::os::raw::c_int,

    pub MTensor_getInteger: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: *mut mint,
    ) -> ::std::os::raw::c_int,

    pub MTensor_getReal: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: *mut mreal,
    ) -> ::std::os::raw::c_int,

    pub MTensor_getComplex: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: *mut mcomplex,
    ) -> ::std::os::raw::c_int,

    pub MTensor_getMTensor: unsafe extern "C" fn(
        arg1: MTensor,
        arg2: *mut mint,
        arg3: mint,
        arg4: *mut MTensor,
    ) -> ::std::os::raw::c_int,

    pub MTensor_getRank: unsafe extern "C" fn(arg1: MTensor) -> mint,
    pub MTensor_getDimensions: unsafe extern "C" fn(arg1: MTensor) -> *const mint,
    pub MTensor_getType: unsafe extern "C" fn(arg1: MTensor) -> mint,
    pub MTensor_getFlattenedLength: unsafe extern "C" fn(arg1: MTensor) -> mint,
    pub MTensor_getIntegerData: unsafe extern "C" fn(arg1: MTensor) -> *mut mint,
    pub MTensor_getRealData: unsafe extern "C" fn(arg1: MTensor) -> *mut mreal,
    pub MTensor_getComplexData: unsafe extern "C" fn(arg1: MTensor) -> *mut mcomplex,

    pub Message: unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char),
    pub AbortQ: unsafe extern "C" fn() -> mint,

    pub getWSLINK: unsafe extern "C" fn(arg1: sys::WolframLibraryData) -> WSLINK,
    pub processWSLINK: unsafe extern "C" fn(arg1: WSLINK) -> ::std::os::raw::c_int,

    pub evaluateExpression: unsafe extern "C" fn(
        arg1: sys::WolframLibraryData,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: mint,
        arg5: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,

    pub runtimeData: *mut st_WolframRuntimeData,

    pub compileLibraryFunctions: *mut st_WolframCompileLibrary_Functions,

    pub VersionNumber: mint,

    pub registerInputStreamMethod: unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        ctor: Option<
            unsafe extern "C" fn(
                arg1: MInputStream,
                msgHead: *const ::std::os::raw::c_char,
                optionsIn: *mut ::std::os::raw::c_void,
            ),
        >,
        handlerTest: Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
            ) -> mbool,
        >,
        methodData: *mut ::std::os::raw::c_void,
        destroyMethod: Option<
            unsafe extern "C" fn(methodData: *mut ::std::os::raw::c_void),
        >,
    ) -> mbool,

    pub unregisterInputStreamMethod:
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> mbool,

    pub registerOutputStreamMethod: unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        ctor: Option<
            unsafe extern "C" fn(
                arg1: MOutputStream,
                msgHead: *const ::std::os::raw::c_char,
                optionsIn: *mut ::std::os::raw::c_void,
                appendMode: mbool,
            ),
        >,
        handlerTest: Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
            ) -> mbool,
        >,
        methodData: *mut ::std::os::raw::c_void,
        destroyMethod: Option<
            unsafe extern "C" fn(methodData: *mut ::std::os::raw::c_void),
        >,
    ) -> mbool,

    pub unregisterOutputStreamMethod:
        unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> mbool,

    pub ioLibraryFunctions: *mut st_WolframIOLibrary_Functions,

    pub getWSLINKEnvironment:
        unsafe extern "C" fn(arg1: sys::WolframLibraryData) -> WSENV,

    pub sparseLibraryFunctions: *mut st_WolframSparseLibrary_Functions,

    pub imageLibraryFunctions: *mut st_WolframImageLibrary_Functions,

    pub registerLibraryExpressionManager: unsafe extern "C" fn(
        mname: *const ::std::os::raw::c_char,
        mfun: Option<
            unsafe extern "C" fn(arg1: sys::WolframLibraryData, arg2: mbool, arg3: mint),
        >,
    )
        -> ::std::os::raw::c_int,

    pub unregisterLibraryExpressionManager: unsafe extern "C" fn(
        mname: *const ::std::os::raw::c_char,
    )
        -> ::std::os::raw::c_int,

    pub releaseManagedLibraryExpression: unsafe extern "C" fn(
        mname: *const ::std::os::raw::c_char,
        id: mint,
    )
        -> ::std::os::raw::c_int,

    pub registerLibraryCallbackManager: unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
        mfun: Option<
            unsafe extern "C" fn(
                arg1: sys::WolframLibraryData,
                arg2: mint,
                arg3: MTensor,
            ) -> mbool,
        >,
    )
        -> ::std::os::raw::c_int,

    pub unregisterLibraryCallbackManager: unsafe extern "C" fn(
        name: *const ::std::os::raw::c_char,
    )
        -> ::std::os::raw::c_int,

    pub callLibraryCallbackFunction: unsafe extern "C" fn(
        id: mint,
        ArgC: mint,
        Args: *mut MArgument,
        Res: MArgument,
    ) -> ::std::os::raw::c_int,

    pub releaseLibraryCallbackFunction:
        unsafe extern "C" fn(id: mint) -> ::std::os::raw::c_int,

    pub validatePath: unsafe extern "C" fn(
        path: *mut ::std::os::raw::c_char,
        type_: ::std::os::raw::c_char,
    ) -> mbool,

    pub protectedModeQ: unsafe extern "C" fn() -> mbool,
    pub rawarrayLibraryFunctions: *mut st_WolframRawArrayLibrary_Functions,
    pub numericarrayLibraryFunctions: *mut st_WolframNumericArrayLibrary_Functions,
    pub setParallelThreadNumber:
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pub restoreParallelThreadNumber: unsafe extern "C" fn(arg1: ::std::os::raw::c_int),
    pub getParallelThreadNumber: unsafe extern "C" fn() -> ::std::os::raw::c_int,
}

/// # Safety
///
/// The `WolframLibraryData` stucture contains function pointers to functions in the
/// Wolfram Runtime Library (RTL). Sending function pointers to another thread is not
/// dangerous; but calling some of the `unsafe` functions from that thread may be.
/// Therefore, this type should be [`Send`].
///
/// Not all of the functions in the Wolfram RTL are safe to call from any thread other
/// than the main Kernel thread. Therefore, the presense of an instance of
/// `WolframLibraryData` on a thread other than the main Kernel thread does not imply that
/// it is safe to call all of the functions listed in this structure from that thread.
/// Each function is marked unsafe, and has independent safety considerations.
unsafe impl Send for WolframLibraryData {}
unsafe impl Sync for WolframLibraryData {}

macro_rules! unwrap_fields {
    ($raw:expr, $data:expr, [ $($field:ident),+ ]) => {{
        WolframLibraryData {
            raw_library_data: $raw,
            VersionNumber: $data.VersionNumber,
            runtimeData: $data.runtimeData,
            compileLibraryFunctions: $data.compileLibraryFunctions,
            rawarrayLibraryFunctions: $data.rawarrayLibraryFunctions,
            numericarrayLibraryFunctions: $data.numericarrayLibraryFunctions,
            sparseLibraryFunctions: $data.sparseLibraryFunctions,
            imageLibraryFunctions: $data.imageLibraryFunctions,
            ioLibraryFunctions: $data.ioLibraryFunctions,
            $($field: $data.$field.unwrap()),+,
        }
    }}
}

impl WolframLibraryData {
    /// Construct a new `WolframLibraryData` from a [`wolfram_library_link_sys::WolframLibraryData`].
    pub fn new(data_ptr: sys::WolframLibraryData) -> Result<Self, ()> {
        if data_ptr.is_null() {
            return Err(());
        }

        let data: sys::st_WolframLibraryData = unsafe { *data_ptr };

        Ok(unwrap_fields!(data_ptr, data, [
            UTF8String_disown,
            MTensor_new,
            MTensor_free,
            MTensor_clone,
            MTensor_shareCount,
            MTensor_disown,
            MTensor_disownAll,
            MTensor_setInteger,
            MTensor_setReal,
            MTensor_setComplex,
            MTensor_setMTensor,
            MTensor_getInteger,
            MTensor_getReal,
            MTensor_getComplex,
            MTensor_getMTensor,
            MTensor_getRank,
            MTensor_getDimensions,
            MTensor_getType,
            MTensor_getFlattenedLength,
            MTensor_getIntegerData,
            MTensor_getRealData,
            MTensor_getComplexData,
            Message,
            AbortQ,
            getWSLINK,
            processWSLINK,
            evaluateExpression,
            registerInputStreamMethod,
            unregisterInputStreamMethod,
            registerOutputStreamMethod,
            unregisterOutputStreamMethod,
            getWSLINKEnvironment,
            registerLibraryExpressionManager,
            unregisterLibraryExpressionManager,
            releaseManagedLibraryExpression,
            registerLibraryCallbackManager,
            unregisterLibraryCallbackManager,
            callLibraryCallbackFunction,
            releaseLibraryCallbackFunction,
            validatePath,
            protectedModeQ,
            setParallelThreadNumber,
            restoreParallelThreadNumber,
            getParallelThreadNumber
        ]))
    }
}
