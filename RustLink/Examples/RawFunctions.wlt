Needs["MUnit`"]

(*======================================*)
(* Raw LibraryLink Functions            *)
(*======================================*)

Test[
	func = LibraryFunctionLoad[
		"libraw_librarylink_function",
		"demo_wxf_byte_array",
		{},
		LibraryDataType[ByteArray]
	];

	data = func[];
	{data, BinaryDeserialize[data]}
	,
	{
		ByteArray["ODpmA3MLQXNzb2NpYXRpb25mAnMEUnVsZVMBYUMBZgJzBFJ1bGVTAWJDAmYCcwRSdWxlUwFjQwM="],
		<| "a" -> 1, "b" -> 2, "c" -> 3|>
	}
]

Test[
	func = LibraryFunctionLoad[
		"libraw_librarylink_function",
		"demo_wxf_safe_byte_array",
		{},
		LibraryDataType[ByteArray]
	];

	data = func[];
	{data, BinaryDeserialize[data]}
	,
	{
		ByteArray["ODpmA3MLQXNzb2NpYXRpb25mAnMEUnVsZVMBYUMBZgJzBFJ1bGVTAWJDAmYCcwRSdWxlUwFjQwM="],
		<| "a" -> 1, "b" -> 2, "c" -> 3|>
	}
]

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