Needs["MUnit`"]

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_create_string", {}, "Void"
		][];
		Global`$ReturnValue
	),
	{1, "two", 3.5}
]

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_create_symbols", {}, "Void"
		][];
		Global`$ReturnValue
	),
	{Global`Example1, Global`Example2, Example3`Example4}
]

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_create_heterogenous", {}, "Void"
		][];
		Global`$ReturnValue
	),
	{1, 2.01, "three", Four, {"a", "b", "c"}}
]

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_evaluate", {}, "Void"
		][];
		Global`$ReturnValue
	),
	4
]

(*====================================*)
(* Custom DownCode                    *)
(*====================================*)

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_custom_downcode", {}, "Void"
		][];

		Global`CustomDownCode[]
	),
	"CUSTOM DOWNCODE"
]