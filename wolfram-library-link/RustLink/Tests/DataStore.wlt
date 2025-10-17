Needs["MUnit`"]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_empty_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_single_int_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[1]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_multiple_int_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[1, 2, 3]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_unnamed_heterogenous_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[1, 2.0, "hello"]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_named_heterogenous_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore["an i64" -> 1, "an f64" -> 2.0, "a str" -> "hello"
		]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_named_and_unnamed_heterogenous_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[1, "real" -> 2.0, "hello" -> "world"]
]

(*====================================*)

(* Non-atomic types                   *)

(*====================================*)

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_named_numeric_array_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore["array" -> NumericArray[{1, 2, 3}, "Integer64"]]
		
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_nested_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore["is_inner" -> False, Developer`DataStore["is_inner"
		 -> True]]
]

Test[
	func = LibraryFunctionLoad["liblibrary_tests", "test_iterated_nested_data_store",
		 {}, "DataStore"];
	func[]
	,
	Developer`DataStore[Developer`DataStore[Developer`DataStore[Developer`DataStore[
		"level" -> 0], "level" -> 1], "level" -> 2]]
]

(*====================================*)

(* DataStore arguments                *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "test_data_store_arg", {"DataStore"
		}, Integer][Developer`DataStore["a", "b", "c"]]
	,
	3
]

(*====================================*)

(* DataStore nodes                    *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "test_data_store_nodes", {},
		 "Void"][]
	,
	Null
]

(*====================================*)

(* u64 support                        *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_u64", {Integer
		}, "DataStore"][42]
	,
	Developer`DataStore[42]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_u64", {"DataStore"
		}, Integer][Developer`DataStore[123]]
	,
	123
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_add_too_large_u64", {}, 
		"DataStore"][]
	,
	LibraryFunctionError["LIBRARY_USER_ERROR", 1002]
	,
	{LibraryFunction::rterr}
]

(*====================================*)

(* usize support                      *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_usize", {Integer
		}, "DataStore"][77]
	,
	Developer`DataStore[77]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_usize", {"DataStore"
		}, Integer][Developer`DataStore[321]]
	,
	321
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_add_too_large_usize", {},
		 "Void"][]
	,
	LibraryFunctionError["LIBRARY_USER_ERROR", 1002]
	,
	{LibraryFunction::rterr}
]

(*====================================*)

(* u32/u16/u8 support                 *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_u32", {Integer
		}, "DataStore"][42]
	,
	Developer`DataStore[42]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_u32", {"DataStore"
		}, Integer][Developer`DataStore[123]]
	,
	123
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_u16", {Integer
		}, "DataStore"][13]
	,
	Developer`DataStore[13]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_u16", {"DataStore"
		}, Integer][Developer`DataStore[7]]
	,
	7
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_u8", {Integer
		}, "DataStore"][5]
	,
	Developer`DataStore[5]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_u8", {"DataStore"
		}, Integer][Developer`DataStore[9]]
	,
	9
]

(*====================================*)

(* i32/i16/i8 support                 *)

(*====================================*)

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_i32", {Integer
		}, "DataStore"][42]
	,
	Developer`DataStore[42]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_i32", {"DataStore"
		}, Integer][Developer`DataStore[123]]
	,
	123
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_i16", {Integer
		}, "DataStore"][13]
	,
	Developer`DataStore[13]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_i16", {"DataStore"
		}, Integer][Developer`DataStore[7]]
	,
	7
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_round_trip_i8", {Integer
		}, "DataStore"][5]
	,
	Developer`DataStore[5]
]

Test[
	LibraryFunctionLoad["liblibrary_tests", "ds_first_as_i8", {"DataStore"
		}, Integer][Developer`DataStore[9]]
	,
	9
]
