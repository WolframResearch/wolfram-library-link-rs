Needs["MUnit`"]

TestMatch[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_wstp_fn_empty",
		LinkObject,
		LinkObject
	][]
	,
	(* The empty arguments list is never read, so it's left on the link and assumed to be
	   the return value. *)
	{}
]

TestMatch[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_wstp_fn_panic_immediately",
		LinkObject,
		LinkObject
	][]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "successful panic"|>,
		(* Avoid hard-coding the panic line/column number into the test. *)
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/tests/test_wstp.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]

TestMatch[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_wstp_fn_poison_link_and_panic",
		LinkObject,
		LinkObject
	][]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "successful panic"|>,
		(* Avoid hard-coding the panic line/column number into the test. *)
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/tests/test_wstp.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]