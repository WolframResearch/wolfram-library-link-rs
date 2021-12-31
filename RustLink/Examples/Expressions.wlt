Needs["MUnit`"]

TestMatch[
	func = LibraryFunctionLoad[
		"libbasic_expressions",
		"echo_arguments",
		LinkObject,
		LinkObject
	];

	func[2, 2]
	,
	(* "finished echoing 2 argument(s)" *)
	(* FIXME: This output is a bug. Fix the bug and update this test case. *)
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|
			"message" -> "WstpFunction: WSTP error reading argument List expression: WSTP error: symbol name 'List' has no context"
		|>,
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/src/args.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]

TestMatch[
	func = LibraryFunctionLoad[
		"libstructured_expressions",
		"sum_of_numbers",
		LinkObject,
		LinkObject
	];

	func[1, 2, 3]
	,
	(* FIXME: This output is a bug. Fix the bug and update this test case. *)
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|
			"message" -> "WstpFunction: WSTP error reading argument List expression: WSTP error: symbol name 'List' has no context"
		|>,
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/src/args.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]