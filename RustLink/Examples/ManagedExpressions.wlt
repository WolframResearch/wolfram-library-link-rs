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