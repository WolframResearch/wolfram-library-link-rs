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
			"message" -> "evaluate(): evaluation of expression failed: WSTP error: symbol name 'ReturnPacket' has no context: \n\texpression: System`Echo[2]"
		|>,
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/src/lib.rs:"],
		"Backtrace" -> Missing["NotEnabled"]
	|>]
]
