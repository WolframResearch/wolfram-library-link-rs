Needs["MUnit`"]

Test[
	func = LibraryFunctionLoad[
		"libbasic_expressions",
		"echo_arguments_wrapper",
		LinkObject,
		LinkObject
	];

	func[2, 2]
	,
	(* "finished echoing 2 argument(s)" *)
	(* FIXME: This output is a bug. Fix the bug and update this test case. *)
	Failure["LibraryFunctionWSTPError", <|
		(* "Message" -> "WSTP error: Symbol name `List` has no context" *)
	|>]
]