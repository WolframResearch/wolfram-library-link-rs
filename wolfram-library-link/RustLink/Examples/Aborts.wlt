Needs["MUnit`"]

Test[
    (* Use TimeConstrained to start an abort that will trigger after we've started
       executing the Rust library function. *)
    TimeConstrained[
        LibraryFunctionLoad[
            "libaborts",
            "wait_for_abort",
            {},
            Integer
        ][]
        ,
        (* Wait for a tenth of a second. *)
        0.25
    ]
    ,
    $Aborted
]