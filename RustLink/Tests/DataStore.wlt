
Needs["MUnit`"]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_empty_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_single_int_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[1]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_multiple_int_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[1, 2, 3]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_unnamed_heterogenous_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[1, 2.0, "hello"]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_named_heterogenous_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[
		"an i64" -> 1,
		"an f64" -> 2.0,
		"a str" -> "hello"
	]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_named_and_unnamed_heterogenous_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[1, "real" -> 2.0, "hello" -> "world"]
]

(*====================================*)
(* Non-atomic types                   *)
(*====================================*)

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_named_numeric_array_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[
		"array" -> NumericArray[{1, 2, 3}, "Integer64"]
	]
]

Test[
	func = LibraryFunctionLoad[
		"liblibrary_tests",
		"test_nested_data_store",
		{},
		"DataStore"
	];

	func[]
	,
	Developer`DataStore[
		"is_inner" -> False,
		Developer`DataStore["is_inner" -> True]
	]
]