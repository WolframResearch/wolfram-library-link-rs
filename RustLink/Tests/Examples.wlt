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

Test[
	func = LibraryFunctionLoad[
		"libnumeric_arrays",
		"sum_int_numeric_array",
		{NumericArray},
		Integer
	];

	{
		func[NumericArray[Range[10], "Integer64"]],
		func[NumericArray[Range[255], "UnsignedInteger8"]]
	}
	,
	{
		55,
		32640
	}
]

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