VerificationTest[
	loader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_native", {"UTF8String"}, "DataStore"];
	meta = loader["libbasic_types"];
	Head[meta] === Developer`DataStore
,
	True,
	TestID -> "NativeLoader"
]
