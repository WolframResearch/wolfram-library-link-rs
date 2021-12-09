//! Lazy bindings to Wolfram Runtime Library (RTL) functions.
//!
//! Attempting to call these bindings will result in a panic if
//! [`initialize()`][crate::initialize] has not been called.

use std::{ffi::c_void, os::raw::c_int};

use once_cell::sync::Lazy;

use crate::sys::{
    self, colorspace_t, errcode_t, imagedata_t, mbool, mcomplex, mint, mreal,
    numericarray_convert_method_t, numericarray_data_t, raw_t_bit, raw_t_real32,
    raw_t_real64, raw_t_ubit16, raw_t_ubit8, type_t, DataStore, DataStoreNode, MArgument,
    MImage, MInputStream, MNumericArray, MOutputStream, MRawArray, MSparseArray, MTensor,
    WSENV, WSLINK,
};

// TODO: Include auto-generated doc comment with path to appropriate field.
//       Mention that these functions are looked-up dynamically using get_library_data().
macro_rules! rtl_func {
    ($($vis:vis $path:ident : $type:ty,)*) => {
        $(
            #[allow(missing_docs, non_upper_case_globals)]
            $vis static $path: Lazy<$type> = Lazy::new(
                || crate::get_library_data().$path
            );
        )*
    };

    ($group:ident => [$($vis:vis $path:ident : $type:ty,)*]) => {
        // NOTE: That these fields are even an Option is likely just bindgen being
        //       conservative with function pointers possibly being null.
        // TODO: Investigate making bindgen treat these as non-null fields?
        $(
            #[allow(missing_docs, non_upper_case_globals)]
            $vis static $path: Lazy<$type> = Lazy::new(
                || unsafe { (*crate::get_library_data().$group) }.$path.expect(concat!("unwrap: ", stringify!($group)))
            );
        )*
    };
}

//======================================
// WolframLibraryData.* fields
//======================================

rtl_func![
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


    pub getWSLINKEnvironment:
        unsafe extern "C" fn(arg1: sys::WolframLibraryData) -> WSENV,

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
    pub setParallelThreadNumber:
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    pub restoreParallelThreadNumber: unsafe extern "C" fn(arg1: ::std::os::raw::c_int),
    pub getParallelThreadNumber: unsafe extern "C" fn() -> ::std::os::raw::c_int,
];

//======================================
// IO Library
//======================================

rtl_func![
    ioLibraryFunctions => [
        pub createAsynchronousTaskWithoutThread: unsafe extern "C" fn() -> mint,
        pub createAsynchronousTaskWithThread:
            unsafe extern "C" fn(
                asyncRunner: ::std::option::Option<
                    unsafe extern "C" fn(
                        asyncTaskID: mint,
                        initData: *mut ::std::os::raw::c_void,
                    ),
                >,
                initData: *mut ::std::os::raw::c_void,
            ) -> mint,
        pub raiseAsyncEvent:
            unsafe extern "C" fn(
                asyncTaskID: mint,
                eventType: *mut ::std::os::raw::c_char,
                arg1: DataStore,
            ),
        pub asynchronousTaskAliveQ: unsafe extern "C" fn(asyncTaskID: mint) -> mbool,
        pub asynchronousTaskStartedQ: unsafe extern "C" fn(asyncTaskID: mint) -> mbool,
        pub createDataStore: unsafe extern "C" fn() -> DataStore,
        pub DataStore_addInteger: unsafe extern "C" fn(arg1: DataStore, arg2: mint),
        pub DataStore_addReal: unsafe extern "C" fn(arg1: DataStore, arg2: mreal),
        pub DataStore_addComplex: unsafe extern "C" fn(arg1: DataStore, arg2: mcomplex),
        pub DataStore_addString: unsafe extern "C" fn(arg1: DataStore, arg2: *mut ::std::os::raw::c_char),
        pub DataStore_addMTensor: unsafe extern "C" fn(arg1: DataStore, arg2: MTensor),
        pub DataStore_addMRawArray: unsafe extern "C" fn(arg1: DataStore, arg2: MRawArray),
        pub DataStore_addMImage: unsafe extern "C" fn(arg1: DataStore, arg2: MImage),
        pub DataStore_addDataStore: unsafe extern "C" fn(arg1: DataStore, arg2: DataStore),
        pub DataStore_addNamedInteger:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: mint,
            ),
        pub DataStore_addNamedReal:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: mreal,
            ),
        pub DataStore_addNamedComplex:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: mcomplex,
            ),
        pub DataStore_addNamedString:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: *mut ::std::os::raw::c_char,
            ),
        pub DataStore_addNamedMTensor:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: MTensor,
            ),
        pub DataStore_addNamedMRawArray:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: MRawArray,
            ),
        pub DataStore_addNamedMImage:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: MImage,
            ),
        pub DataStore_addNamedDataStore:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: DataStore,
            ),
        pub removeAsynchronousTask: unsafe extern "C" fn(asyncTaskID: mint) -> mint,
        pub deleteDataStore: unsafe extern "C" fn(arg1: DataStore),
        pub copyDataStore: unsafe extern "C" fn(arg1: DataStore) -> DataStore,
        pub DataStore_getLength: unsafe extern "C" fn(arg1: DataStore) -> mint,
        pub DataStore_getFirstNode: unsafe extern "C" fn(arg1: DataStore) -> DataStoreNode,
        pub DataStore_getLastNode: unsafe extern "C" fn(arg1: DataStore) -> DataStoreNode,
        pub DataStoreNode_getNextNode: unsafe extern "C" fn(arg1: DataStoreNode) -> DataStoreNode,
        pub DataStoreNode_getDataType: unsafe extern "C" fn(arg1: DataStoreNode) -> type_t,
        pub DataStoreNode_getData: unsafe extern "C" fn(arg1: DataStoreNode, arg2: *mut MArgument) -> errcode_t,
        pub DataStoreNode_getName:
            unsafe extern "C" fn(
                arg1: DataStoreNode,
                arg2: *mut *mut ::std::os::raw::c_char,
            ) -> errcode_t,
        pub DataStore_addBoolean: unsafe extern "C" fn(arg1: DataStore, arg2: mbool),
        pub DataStore_addNamedBoolean:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: mbool,
            ),
        pub DataStore_addMNumericArray: unsafe extern "C" fn(arg1: DataStore, arg2: MNumericArray),
        pub DataStore_addNamedMNumericArray:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: MNumericArray,
            ),
        pub DataStore_addMSparseArray: unsafe extern "C" fn(arg1: DataStore, arg2: MSparseArray),
        pub DataStore_addNamedMSparseArray:
            unsafe extern "C" fn(
                arg1: DataStore,
                arg2: *mut ::std::os::raw::c_char,
                arg3: MSparseArray,
            ),
    ]
];

//======================================
// NumericArray Library
//======================================

rtl_func![
    numericarrayLibraryFunctions => [
        pub MNumericArray_new: unsafe extern "C" fn(arg1: numericarray_data_t, arg2: mint, arg3: *const mint, arg4: *mut MNumericArray) -> errcode_t,
        pub MNumericArray_free: unsafe extern "C" fn(arg1: MNumericArray),
        pub MNumericArray_clone: unsafe extern "C" fn(arg1: MNumericArray, arg2: *mut MNumericArray) -> errcode_t,
        pub MNumericArray_disown: unsafe extern "C" fn(arg1: MNumericArray),
        pub MNumericArray_disownAll: unsafe extern "C" fn(arg1: MNumericArray),
        pub MNumericArray_shareCount: unsafe extern "C" fn(arg1: MNumericArray) -> mint,
        pub MNumericArray_getType: unsafe extern "C" fn(arg1: MNumericArray) -> numericarray_data_t,
        pub MNumericArray_getRank: unsafe extern "C" fn(arg1: MNumericArray) -> mint,
        pub MNumericArray_getDimensions: unsafe extern "C" fn(arg1: MNumericArray) -> *const mint,
        pub MNumericArray_getFlattenedLength: unsafe extern "C" fn(arg1: MNumericArray) -> mint,
        pub MNumericArray_getData: unsafe extern "C" fn(arg1: MNumericArray) -> *mut c_void,
        pub MNumericArray_convertType: unsafe extern "C" fn(arg1: *mut MNumericArray, arg2: MNumericArray, arg3: numericarray_data_t, arg4: numericarray_convert_method_t, arg5: mreal) -> errcode_t,
    ]
];

//======================================
// Image Library
//======================================

rtl_func![
    imageLibraryFunctions => [
        pub MImage_new2D: unsafe extern "C" fn(arg1: mint, arg2: mint, arg3: mint, arg4: imagedata_t, arg5: colorspace_t, arg6: mbool, arg7: *mut MImage) -> c_int,
        pub MImage_new3D: unsafe extern "C" fn(arg1: mint, arg2: mint, arg3: mint, arg4: mint, arg5: imagedata_t, arg6: colorspace_t, arg7: mbool, arg8: *mut MImage) -> c_int,
        pub MImage_clone: unsafe extern "C" fn(arg1: MImage, arg2: *mut MImage) -> c_int,
        pub MImage_free: unsafe extern "C" fn(arg1: MImage),
        pub MImage_disown: unsafe extern "C" fn(arg1: MImage),
        pub MImage_disownAll: unsafe extern "C" fn(arg1: MImage),
        pub MImage_shareCount: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getDataType: unsafe extern "C" fn(arg1: MImage) -> imagedata_t,
        pub MImage_getRowCount: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getColumnCount: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getSliceCount: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getRank: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getChannels: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_alphaChannelQ: unsafe extern "C" fn(arg1: MImage) -> mbool,
        pub MImage_interleavedQ: unsafe extern "C" fn(arg1: MImage) -> mbool,
        pub MImage_getColorSpace: unsafe extern "C" fn(arg1: MImage) -> colorspace_t,
        pub MImage_getFlattenedLength: unsafe extern "C" fn(arg1: MImage) -> mint,
        pub MImage_getBit: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: *mut raw_t_bit) -> c_int,
        pub MImage_getByte: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: *mut raw_t_ubit8) -> c_int,
        pub MImage_getBit16: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: *mut raw_t_ubit16) -> c_int,
        pub MImage_getReal32: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: *mut raw_t_real32) -> c_int,
        pub MImage_getReal: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: *mut raw_t_real64) -> c_int,
        pub MImage_setBit: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: raw_t_bit) -> c_int,
        pub MImage_setByte: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: raw_t_ubit8) -> c_int,
        pub MImage_setBit16: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: raw_t_ubit16) -> c_int,
        pub MImage_setReal32: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: raw_t_real32) -> c_int,
        pub MImage_setReal: unsafe extern "C" fn(arg1: MImage, arg2: *mut mint, arg3: mint, arg4: raw_t_real64) -> c_int,
        pub MImage_getRawData: unsafe extern "C" fn(arg1: MImage) -> *mut c_void,
        pub MImage_getBitData: unsafe extern "C" fn(arg1: MImage) -> *mut raw_t_bit,
        pub MImage_getByteData: unsafe extern "C" fn(arg1: MImage) -> *mut raw_t_ubit8,
        pub MImage_getBit16Data: unsafe extern "C" fn(arg1: MImage) -> *mut raw_t_ubit16,
        pub MImage_getReal32Data: unsafe extern "C" fn(arg1: MImage) -> *mut raw_t_real32,
        pub MImage_getRealData: unsafe extern "C" fn(arg1: MImage) -> *mut raw_t_real64,
        pub MImage_convertType: unsafe extern "C" fn(arg1: MImage, arg2: imagedata_t, arg3: mbool) -> MImage,
    ]
];

//======================================
// Sparse Library
//======================================

rtl_func![
    sparseLibraryFunctions => [
        pub MSparseArray_clone: unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MSparseArray) -> c_int,
        pub MSparseArray_free: unsafe extern "C" fn(arg1: MSparseArray),
        pub MSparseArray_disown: unsafe extern "C" fn(arg1: MSparseArray),
        pub MSparseArray_disownAll: unsafe extern "C" fn(arg1: MSparseArray),
        pub MSparseArray_shareCount: unsafe extern "C" fn(arg1: MSparseArray) -> mint,
        pub MSparseArray_getRank: unsafe extern "C" fn(arg1: MSparseArray) -> mint,
        pub MSparseArray_getDimensions: unsafe extern "C" fn(arg1: MSparseArray) -> *const mint,
        pub MSparseArray_getImplicitValue: unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor,
        pub MSparseArray_getExplicitValues: unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor,
        pub MSparseArray_getRowPointers: unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor,
        pub MSparseArray_getColumnIndices: unsafe extern "C" fn(arg1: MSparseArray) -> *mut MTensor,
        pub MSparseArray_getExplicitPositions: unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MTensor) -> c_int,
        pub MSparseArray_resetImplicitValue: unsafe extern "C" fn(arg1: MSparseArray, arg2: MTensor, arg3: *mut MSparseArray) -> c_int,
        pub MSparseArray_toMTensor: unsafe extern "C" fn(arg1: MSparseArray, arg2: *mut MTensor) -> c_int,
        pub MSparseArray_fromMTensor: unsafe extern "C" fn(arg1: MTensor, arg2: MTensor, arg3: *mut MSparseArray) -> c_int,
        pub MSparseArray_fromExplicitPositions: unsafe extern "C" fn(arg1: MTensor, arg2: MTensor, arg3: MTensor, arg4: MTensor, arg5: *mut MSparseArray) -> c_int,
    ]
];


// pub compileLibraryFunctions: *mut st_WolframCompileLibrary_Functions,
// pub rawarrayLibraryFunctions: *mut st_WolframRawArrayLibrary_Functions,
