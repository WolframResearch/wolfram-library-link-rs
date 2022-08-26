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
		"test_wstp_fn_panic_immediately_with_formatting",
		LinkObject,
		LinkObject
	][]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "successful formatted panic"|>,
		(* Avoid hard-coding the panic line/column number into the test. *)
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/tests/test_wstp.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]

TestMatch[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_wstp_fn_panic_partial_result",
		LinkObject,
		LinkObject
	][]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "incomplete result"|>,
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

TestMatch[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_wstp_panic_with_empty_link",
		LinkObject,
		LinkObject
	][]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "panic while !link.is_ready()"|>,
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/tests/test_wstp.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]

(*====================================*)
(* Vec<Expr>                          *)
(*====================================*)

TestMatch[
	Block[{$Context = "UnusedContext`", $ContextPath = {}},
		LibraryFunctionLoad[
			"liblibrary_tests",
			"test_wstp_expr_return_null",
			LinkObject,
			LinkObject
		][]
	]
	,
	Null
]