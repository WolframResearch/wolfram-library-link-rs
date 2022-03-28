Needs["MUnit`"]

Test[
    Block[{$Context = "UnlikelyContext`", $ContextPath = {}},
        LibraryFunctionLoad[
            "liblibrary_tests", "test_runtime_function_from_main_thread", {}, "Boolean"
        ][]
    ]
    ,
    True
]

Test[
    result = Block[{$Context = "UnlikelyContext`", $ContextPath = {}},
        LibraryFunctionLoad[
            "liblibrary_tests", "test_runtime_function_from_non_main_thread", {}, String
        ][]
    ];

    StringQ[result] && StringStartsQ[
        result,
        "PANIC: error: attempted to call back into the Wolfram Kernel from a non-main thread at"
    ]
]