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