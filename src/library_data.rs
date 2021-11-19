use std::sync::Mutex;

use crate::sys::{
    self, mbool, mcomplex, mint, mreal, st_WolframCompileLibrary_Functions,
    st_WolframIOLibrary_Functions, st_WolframImageLibrary_Functions,
    st_WolframNumericArrayLibrary_Functions, st_WolframRawArrayLibrary_Functions,
    st_WolframRuntimeData, st_WolframSparseLibrary_Functions, MArgument, MInputStream,
    MOutputStream, MTensor, WSENV, WSLINK,
};

thread_local! {
    static LIBRARY_DATA: Mutex<Option<WolframLibraryData>> = Mutex::new(None);
}

/// Initialize static data for the current Wolfram library.
///
/// This function should be called during by the [initialization hook][lib-init]
/// provided by this library.
///
/// # Example
///
/// When a dynamic library is loaded by the Wolfram Language (for example, via
/// [`LibraryFunctionLoad`](https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html)),
/// the system will call the function `WolframLibrary_initialize()` if one is provided by
/// the library. This function is used to initialize callbacks from the library into
/// functions provided by the Wolfram runtime.
///
/// ```
/// use std::os::raw::c_int;
/// use wl_library_link::{sys, initialize, WolframLibraryData};
///
/// #[no_mangle]
/// extern "C" fn WolframLibrary_initialize(data: sys::WolframLibraryData) -> c_int {
///     match WolframLibraryData::new(data) {
///         Ok(data) => {
///             initialize(data);
///             return 0;
///         },
///         Err(_) => return 1,
///     }
/// }
/// ```
///
/// [lib-init]: https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#280210622
pub fn initialize(data: WolframLibraryData) {
    LIBRARY_DATA.with(|static_data| {
        let mut static_data = static_data
            .lock()
            .expect("failed to acquire lock on global Wolfram LIBRARY_DATA");

        *static_data = Some(data);
    });
}

pub(crate) fn get_library_data() -> WolframLibraryData {
    let data = LIBRARY_DATA.with(|static_data| {
        let static_data = static_data
            .lock()
            .expect("failed to acquire lock on global Wolfram LIBRARY_DATA");

        *static_data
    });

    // TODO: Include a comment here mentioning that the library could/should provide a
    //       WolframLibrary_initialize() function which calls initialize_library_data()?
    data.expect(
        "get_library_data: global Wolfram LIBRARY_DATA static is not initialized.",
    )
}

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct WolframLibraryData {
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

macro_rules! unwrap_fields {
    ($data:expr, [ $($field:ident),+ ]) => {{
        WolframLibraryData {
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
    pub fn new(data_ptr: sys::WolframLibraryData) -> Result<Self, ()> {
        if data_ptr.is_null() {
            return Err(());
        }

        let data: sys::st_WolframLibraryData = unsafe { *data_ptr };

        Ok(unwrap_fields!(data, [
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
