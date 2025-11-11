Needs["MUnit`"]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_bigint_roundtrip",
		{String},
		String
	][ToString[123456789012345678901234567890]]
	,
	"123456789012345678901234567891"
]

Test[
	LibraryFunctionLoad[
		"liblibrary_tests",
		"test_biguint_roundtrip",
		{String},
		String
	][ToString[987654321098765432109876543210]]
	,
	"987654321098765432109876543211"
]
