MySymbol::msg = "This is a message generated from ``";

(* FIXME: For some reason, the test below fails with the following message unless
	we _save the result_ of calling Links[]:
		LinkObject::linkd: Unable to communicate with closed link LinkObject[...]
	Note that this only happens when running the tests using
	`wolfram-cli paclet test`, so it's likely this is some unknown conflict.
*)
before = Links[];

VerificationTest[
    generateMessage = LibraryFunctionLoad[
        "libwll_docs", "generate_message",
        LinkObject, LinkObject
    ];

    (* Note:
        Set $Context and $ContextPath to force symbols sent
        via WSTP to include their context. *)
    Block[{$Context = "Empty`", $ContextPath = {}},
        generateMessage[]
    ]
    ,
    Null
    ,
    {HoldForm[Message[MySymbol::msg, "a Rust LibraryLink function"]]}
]