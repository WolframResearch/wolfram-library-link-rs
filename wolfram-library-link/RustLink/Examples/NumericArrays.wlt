Needs["MUnit`"]

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
		"libnumeric_arrays",
		"sum_real_numeric_array",
		{NumericArray},
		Real
	];

	{
		func[NumericArray[Range[1, 10, 1/81], "Real32"]],
		func[NumericArray[Range[1, 10, 1/81], "Real64"]]
	}
	,
	{
		4015.0,
		4015.0
	}
]
