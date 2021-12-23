Needs["MUnit`"]

Test[
	LibraryFunctionLoad[
		"libwstp_example",
		"square_wstp",
		LinkObject,
		LinkObject
	][4]
	,
	16
]

(* Test that passing more than one argument to square_wstp() results in a Failure. *)
TestMatch[
	LibraryFunctionLoad[
		"libwstp_example",
		"square_wstp",
		LinkObject,
		LinkObject
	][4, 4]
	,
	Failure["RustPanic", <|
		"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
		"MessageParameters" -> <|"message" -> "square_wstp: expected to get a single argument"|>,
		"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/wstp.rs:"],
		"Backtrace" -> Sequence[]
	|>]
]

Test[
	LibraryFunctionLoad[
		"libwstp_example",
		"count_args",
		LinkObject,
		LinkObject
	][a, b, c]
	,
	3
]

Test[
	totalArgsI64 = LibraryFunctionLoad[
		"libwstp_example",
		"total_args_i64",
		LinkObject,
		LinkObject
	];

	{
		totalArgsI64[2, 2],
		totalArgsI64[1, 2, 3]
	}
	,
	{
		4,
		6
	}
]

Test[
	stringJoin = LibraryFunctionLoad[
		"libwstp_example",
		"string_join",
		LinkObject,
		LinkObject
	];

	{
		stringJoin["Hello, ", "World!"],
		stringJoin[Sequence @@ CharacterRange["A", "G"]],
		stringJoin[]
	},
	{
		"Hello, World!",
		"ABCDEFG",
		""
	}
]