Needs["MUnit`"]

TestMatch[
    loadFunctions = LibraryFunctionLoad[
        "libmanaged_exprs",
        "load_managed_exprs_functions",
        LinkObject,
        LinkObject
    ];

    $functions = loadFunctions["libmanaged_exprs"] // Sort
    ,
    <|
        "get_instance_data" -> Function[___],
        "set_instance_value" -> Function[___]
    |>
]

Test[
    $obj = CreateManagedLibraryExpression["my_object", MyObject];

    MatchQ[$obj, MyObject[1]]
]

Test[
    ManagedLibraryExpressionQ[$obj]
]

Test[
    $objID = ManagedLibraryExpressionID[$obj];

    MatchQ[$objID, 1]
]

Test[
    $functions["get_instance_data"][$objID]
    ,
    <| "Value" -> "default" |>
]

Test[
    $functions["set_instance_value"][$objID, "new value"]
    ,
    Null
]

Test[
    $functions["get_instance_data"][$objID]
    ,
    <| "Value" -> "new value" |>
]

TestMatch[
    (* Clear $obj. This is the last copy of this managed expression, so the Kernel will
       call managed.rs/manage_instance() with a `ManagedExpressionEvent::Drop(_)` event.

       The fact that `ClearAll[..]` (or $obj going "out of scope" naturally) has the
       effect of calling back into the library to deallocate the object instance is the
       key feature of managed library expressions.
    *)
    ClearAll[$obj];

    $functions["get_instance_data"][$objID]
    ,
    (* Test that trying to access a deallocated instance fails. *)
    Failure["RustPanic", <|
        "MessageTemplate" -> "Rust LibraryLink function panic: `message`",
        "MessageParameters" -> <|"message" -> "instance does not exist"|>,
        "SourceLocation" -> s_?StringQ /; StringStartsQ[s, "wolfram-library-link/examples/exprs/managed.rs:"],
        "Backtrace" -> Missing["NotEnabled"]
    |>]
]