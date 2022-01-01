Needs["MUnit`"]

(*======================================*)
(* Raw WSTP Functions                   *)
(*======================================*)

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
		"libraw_wstp_function",
		"wstp_expr_function",
		LinkObject,
		LinkObject
	];

	func[{1, 2, 3}]
	,
	(* FIXME: This output is a bug. Fix the bug and update this test case. *)
	Failure["WSTP Error", <|
		"Message" -> "WSTP error: symbol name 'List' has no context"
	|>]
]