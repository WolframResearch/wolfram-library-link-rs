Needs["MUnit`"]

Test[
	func = LibraryFunctionLoad[
		"libraw_wstp_function",
		"demo_wstp_function",
		LinkObject,
		LinkObject
	];

	func[2, 2]
	,
	4
]

Test[
	func = LibraryFunctionLoad[
		"libraw_wstp_function",
		"demo_wstp_function_callback",
		LinkObject,
		LinkObject
	];

	func[]
	,
	"returned normally"
]

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
		"Message" -> "WSTP error: Symbol name `List` has no context"
	|>]
]