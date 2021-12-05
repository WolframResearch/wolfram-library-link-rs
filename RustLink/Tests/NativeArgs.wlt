Needs["MUnit`"]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_no_args",
		{},
		Integer
	][]
	,
	4
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_mint",
		{Integer},
		Integer
	][5]
	,
	25
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_raw_mint",
		{Integer},
		Integer
	][9]
	,
	81
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_mint_mint",
		{Integer, Integer},
		Integer
	][5, 10]
	,
	15
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_mreal",
		{Real},
		Real
	][2.5]
	,
	6.25
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_i64",
		{Integer},
		Integer
	][5]
	,
	25
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_i64_i64",
		{Integer, Integer},
		Integer
	][5, 10]
	,
	15
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_f64",
		{Real},
		Real
	][2.5]
	,
	6.25
]

(*---------*)
(* Strings *)
(*---------*)

(* Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_str",
		{String},
		String
	]["hello"]
	,
	"olleh"
] *)

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_string",
		{String},
		String
	]["hello"]
	,
	"olleh"
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_c_string",
		{String},
		Integer
	]["hello world"]
	,
	11
]

(*----------------*)
(* NumericArray's *)
(*----------------*)

Test[
	totalI64 = LibraryFunctionLoad[
		"liblibrary_tests",
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
		"liblibrary_tests",
		"positive_i64",
		{LibraryDataType[NumericArray, "Integer64"]},
		LibraryDataType[NumericArray, "UnsignedInteger8"]
	];

	positiveQ[NumericArray[{0, 1, -2, 3, 4,	-5}, "Integer64"]]
	,
	NumericArray[{0, 1, 0, 1, 1, 0}, "UnsignedInteger8"]
]