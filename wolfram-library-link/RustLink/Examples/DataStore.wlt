Needs["MUnit`"]

Test[
    LibraryFunctionLoad[
        "libdata_store",
        "string_join",
        {"DataStore"},
        String
    ][
        Developer`DataStore["hello", " ", "world"]
    ]
    ,
    "hello world"
]