Needs["MUnit`"]

$NAType = LibraryDataType[NumericArray, "Integer64"]
(* Constructs a fresh NumericArray on each access. *)
$NA := NumericArray[{1, 2, 3}, "Integer64"]

Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_automatic_count",
        {
            {LibraryDataType[NumericArray, "Integer64"], Automatic}
        },
        Integer
    ][$NA]
    ,
    0
]

Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_constant_count",
        {
            {LibraryDataType[NumericArray, "Integer64"], "Constant"}
        },
        Integer
    ][$NA]
    ,
    0
]

Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_manual_count",
        {
            {LibraryDataType[NumericArray, "Integer64"], "Manual"}
        },
        Integer
    ][$NA]
    ,
    0
]

Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_shared_count",
        {
            {LibraryDataType[NumericArray, "Integer64"], "Shared"}
        },
        Integer
    ][$NA]
    ,
    1
]

(* Test passing one NumericArray as two different arguments, using "Constant". *)
Test[
    With[{array = $NA},
        LibraryFunctionLoad[
            "liblibrary_tests",
            "test_na_constant_are_ptr_eq",
            {
                {LibraryDataType[NumericArray, "Integer64"], "Constant"},
                {LibraryDataType[NumericArray, "Integer64"], "Constant"}
            },
            "DataStore"
        ][array, array]
    ]
    ,
    (* The two arrays:
        * should be `ptr_eq()`
        * their `share_count()` should be 0
    *)
    Developer`DataStore[True, 0]
]

(* Test passing one NumericArray as two different arguments, using "Manual". *)
Test[
    With[{array = $NA},
        LibraryFunctionLoad[
            "liblibrary_tests",
            "test_na_manual_are_not_ptr_eq",
            {
                {LibraryDataType[NumericArray, "Integer64"], "Manual"},
                {LibraryDataType[NumericArray, "Integer64"], "Manual"}
            },
            "DataStore"
        ][array, array]
    ]
    ,
    (* The two arrays:
        * should *not* be `ptr_eq()`
        * their `share_count()` should be 0
        * `array1.as_slice_mut().is_some()` should be True
    *)
    Developer`DataStore[False, 0, True]
]

(* Test passing one NumericArray as two different arguments, using "Shared". *)
Test[
    With[{array = $NA},
        LibraryFunctionLoad[
            "liblibrary_tests",
            "test_na_shared_are_ptr_eq",
            {
                {LibraryDataType[NumericArray, "Integer64"], "Shared"},
                {LibraryDataType[NumericArray, "Integer64"], "Shared"}
            },
            "DataStore"
        ][array, array]
    ]
    ,
    (* The two arrays:
        * should be `ptr_eq()`
        * their `share_count()` should be 2
        * `array1.as_slice_mut().is_some()` should be False
    *)
    Developer`DataStore[True, 2, False]
]

(* Test cloning a NumericArray *)
Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_clone",
        {},
        "Boolean"
    ][]
]

(* Test cloning a "Shared" NumericArray *)
Test[
    LibraryFunctionLoad[
        "liblibrary_tests",
        "test_na_shared_clone",
        {
            {LibraryDataType[NumericArray, "Integer64"], "Shared"}
        },
        "Boolean"
    ][$NA]
]