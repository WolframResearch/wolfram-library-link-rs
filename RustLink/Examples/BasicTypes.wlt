Needs["MUnit`"]

Test[
	square = LibraryFunctionLoad[
		"libbasic_types",
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
		"libbasic_types",
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
		"libbasic_types",
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
		"libbasic_types",
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
		"libbasic_types",
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
		"libbasic_types",
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
		"libbasic_types",
		"raw_square",
		{Integer},
		Integer
	];

	rawSquare[50]
	,
	2500
]