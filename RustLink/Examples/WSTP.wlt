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
		"Backtrace" -> Missing["NotEnabled"]
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

TestMatch[
	linkExprIdentity = LibraryFunctionLoad[
		"libwstp_example",
		"link_expr_identity",
		LinkObject,
		LinkObject
	];
	(* Note:
		Set $Context and $ContextPath to force symbols sent across the LinkObject to
		contain the symbol context explicitly.
	*)
	Block[{$Context = "UnusedContext`", $ContextPath = {}},
		linkExprIdentity[foo[], bar[baz]]
	]
	,
	{foo[], bar[baz]}
]

TestMatch[
	exprStringJoin = LibraryFunctionLoad[
		"libwstp_example",
		"expr_string_join",
		LinkObject,
		LinkObject
	];
	(* Note:
		Set $Context and $ContextPath to force symbols sent across the LinkObject to
		contain the symbol context explicitly.
	*)
	Block[{$Context = "UnusedContext`", $ContextPath = {}},
		{
			exprStringJoin[],
			exprStringJoin["Foo"],
			exprStringJoin["Foo", "Bar"],
			exprStringJoin[Sequence @@ CharacterRange["a", "f"]],
			exprStringJoin[1, 2, 3]
		}
	]
	,
	{
		"",
		"Foo",
		"FooBar",
		"abcdef",
		Failure["RustPanic", <|
			"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
			"MessageParameters" -> <|"message" -> "expected String argument, got: 1"|>,
			(* Avoid hard-coding the panic line/column number into the test. *)
			"SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/wstp.rs:"],
			"Backtrace" -> Missing["NotEnabled"]
		|>]
	}
]

TestMatch[
	total = LibraryFunctionLoad[
		"libwstp_example",
		"total",
		LinkObject,
		LinkObject
	];
	(* Note:
		Set $Context and $ContextPath to force symbols sent across the LinkObject to
		contain the symbol context explicitly.
	*)
	Block[{$Context = "UnusedContext`", $ContextPath = {}},
		{
			total[],
			total[1, 2, 3],
			total[1, 2.5, 7],
			(* Cause an integer overflow. *)
			total[2^62, 2^62, 2^62],
			total[5, "Hello"]
		}
	]
	,
	{
		0,
		6,
		10.5,
		Failure["RustPanic", <|
			"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
			"MessageParameters" -> <|"message" -> "attempt to add with overflow"|>,
			"SourceLocation" -> s0_?StringQ /; StringStartsQ[s0, "wolfram-library-link/examples/wstp.rs:232:56"],
			"Backtrace" -> Missing["NotEnabled"]
		|>],
		Failure["RustPanic", <|
			"MessageTemplate" -> "Rust LibraryLink function panic: `message`",
			"MessageParameters" -> <|
				"message" -> "expected argument as position 1 to be a number, got \"Hello\""
			|>,
			"SourceLocation" -> s1_?StringQ /; StringStartsQ[s1, "wolfram-library-link/examples/wstp.rs:222:18"],
			"Backtrace" -> Missing["NotEnabled"]
		|>]
	}
]