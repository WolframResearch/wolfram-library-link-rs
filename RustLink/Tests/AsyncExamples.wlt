Needs["MUnit`"]

(* Test the async_file_watcher_raw.rs example. *)
Test[
    delay = 100;
    file = CreateFile[];

    $changes = {};

    eventHandler[
        taskObject_,
        "change",
        {modTime_}
    ] := AppendTo[$changes, modTime];

    (* Begin the background task. *)
    task = Internal`CreateAsynchronousTask[
        LibraryFunctionLoad[
            "libasync_file_watcher_raw",
            "start_file_watcher",
            {Integer, "UTF8String"},
            Integer
        ],
        {delay, file},
        eventHandler
    ];

    (* Modify the watched file. *)
    Put[1, file];
    expectedModifiedTime = UnixTime[];

    (* Ensure the file modification check has time to run. *)
    Pause[Quantity[2 * delay, "Milliseconds"]];

    StopAsynchronousTask[task];

    $changes
    ,
    {expectedModifiedTime}
]

(* Test the async_file_watcher.rs example. This is identical to the above test, except the
   example implementation uses the safe wrappers. *)
Test[
    delay = 100;
    file = CreateFile[];

    $changes2 = {};

    eventHandler[
        taskObject_,
        "change",
        {modTime_}
    ] := AppendTo[$changes2, modTime];

    (* Begin the background task. *)
    task = Internal`CreateAsynchronousTask[
        LibraryFunctionLoad[
            "libasync_file_watcher",
            "start_file_watcher",
            {Integer, "UTF8String"},
            Integer
        ],
        {delay, file},
        eventHandler
    ];

    (* Modify the watched file. *)
    Put[1, file];
    expectedModifiedTime = UnixTime[];

    (* Ensure the file modification check has time to run. *)
    Pause[Quantity[2 * delay, "Milliseconds"]];

    StopAsynchronousTask[task];

    $changes2
    ,
    {expectedModifiedTime}
]