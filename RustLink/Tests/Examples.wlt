Needs["MUnit`"]

Test[
	square = LibraryFunctionLoad[
		"libnative_data_types",
		"square",
		{Integer},
		Integer
	];

	square[11]
	,
	121
]

Test[
	reverseString = LibraryFunctionLoad[
		"libnative_data_types",
		"reverse_string",
		{String},
		String
	];

	reverseString["hello world"]
	,
	"dlrow olleh"
]

Test[
	add2 = LibraryFunctionLoad[
		"libnative_data_types",
		"add2",
		{Integer, Integer},
		Integer
	];

	add2[3, 3]
	,
	6
]

Test[
	totalI64 = LibraryFunctionLoad[
		"libnative_data_types",
		"total_i64",
		{LibraryDataType[NumericArray, "Integer64"]},
		Integer
	];

	totalI64[NumericArray[Range[100], "Integer64"]]
	,
	5050
]

Test[
	positiveQ = LibraryFunctionLoad[
		"libnative_data_types",
		"positive_i64",
		{LibraryDataType[NumericArray, "Integer64"]},
		LibraryDataType[NumericArray, "UnsignedInteger8"]
	];

	positiveQ[NumericArray[{0, 1, -2, 3, 4,	-5}, "Integer64"]]
	,
	NumericArray[{0, 1, 0, 1, 1, 0}, "UnsignedInteger8"]
]

Test[
	randomNumber = LibraryFunctionLoad[
		"libnative_data_types",
		"xkcd_get_random_number",
		{},
		Integer
	];

	randomNumber[]
	,
	4
]

Test[
	rawSquare = LibraryFunctionLoad[
		"libnative_data_types",
		"raw_square",
		{Integer},
		Integer
	];

	rawSquare[50]
	,
	2500
]

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