Needs["MUnit`"]

Test[
	(
		LibraryFunctionLoad[
			"liblibrary_tests", "test_kernel_expr_create_string", {}, "Void"
		][];
		Global`$ReturnValue
	),
	{1, "two"}
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