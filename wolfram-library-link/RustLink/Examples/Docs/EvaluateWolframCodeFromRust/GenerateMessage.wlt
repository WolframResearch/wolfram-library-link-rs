MySymbol::msg = "This is a message generated from ``";

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