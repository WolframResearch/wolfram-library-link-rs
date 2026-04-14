Needs["MUnit`"]

(*==================================*)
(* Scalars not already in NativeArgs *)
(*==================================*)

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_bool_not", {"Boolean"}, "Boolean"][True],
  False
  , TestID -> "bool-not-true"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_bool_not", {"Boolean"}, "Boolean"][False],
  True
  , TestID -> "bool-not-false"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_char_code", {"UTF8String"}, Integer]["A"],
  65
  , TestID -> "char-code-ascii"
]

(*==================================*)
(* Complex numbers via `mcomplex`   *)
(*==================================*)

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_mcomplex_conj", {Complex}, Complex][3.0 + 4.0 I],
  3.0 - 4.0 I
  , TestID -> "mcomplex-conj"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_mcomplex_conj", {Complex}, Complex][1.0 + 0.0 I],
  1.0 + 0.0 I
  , TestID -> "mcomplex-conj-real-only"
]

(*==================================*)
(* Option<T> via DataStore (None is *)
(* a DataStore with 0 entries).     *)
(*==================================*)

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_option_int_or_negone",
    {"DataStore"}, Integer][Developer`DataStore[42]],
  42
  , TestID -> "option-int-some"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_option_int_or_negone",
    {"DataStore"}, Integer][Developer`DataStore[]],
  -1
  , TestID -> "option-int-none"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_option_string_echo_or_empty",
    {"DataStore"}, String][Developer`DataStore["hello"]],
  "hello"
  , TestID -> "option-string-some"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_option_string_echo_or_empty",
    {"DataStore"}, String][Developer`DataStore[]],
  ""
  , TestID -> "option-string-none"
]

(*==================================*)
(* Vec<T> via DataStore              *)
(*==================================*)

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_vec_i32_sum", {"DataStore"}, Integer][
    Developer`DataStore[1, 2, 3, 4, 5]],
  15
  , TestID -> "vec-i32-sum"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_vec_i64_max", {"DataStore"}, Integer][
    Developer`DataStore[-10, 5, 3, 42, 17]],
  42
  , TestID -> "vec-i64-max"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_vec_f64_sum", {"DataStore"}, Real][
    Developer`DataStore[1.5, 2.5, 3.0]],
  7.0
  , TestID -> "vec-f64-sum"
]

Test[
  Module[{fn = LibraryFunctionLoad["liblibrary_tests", "test_vec_string_lengths",
    {"DataStore"}, "DataStore"]},
    List @@ fn[Developer`DataStore["a", "bb", "ccc"]]
  ],
  {1, 2, 3}
  , TestID -> "vec-string-lengths"
]

(*==================================*)
(* DataStore containing every type  *)
(*==================================*)

Test[
  Module[{fn = LibraryFunctionLoad["liblibrary_tests", "test_datastore_every_type",
    {}, "DataStore"], ds},
    ds = fn[];
    List @@ ds
  ],
  {True, -42, 3.25, "hello"}
  , TestID -> "datastore-every-type"
]

(*==================================*)
(* NumericArray round-trips         *)
(*==================================*)

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_na_u8_identity",
    {LibraryDataType[NumericArray, "UnsignedInteger8"]},
    LibraryDataType[NumericArray, "UnsignedInteger8"]][
    NumericArray[{1, 2, 3}, "UnsignedInteger8"]],
  NumericArray[{1, 2, 3}, "UnsignedInteger8"]
  , TestID -> "numericarray-u8-id"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_na_u16_identity",
    {LibraryDataType[NumericArray, "UnsignedInteger16"]},
    LibraryDataType[NumericArray, "UnsignedInteger16"]][
    NumericArray[{100, 200, 300}, "UnsignedInteger16"]],
  NumericArray[{100, 200, 300}, "UnsignedInteger16"]
  , TestID -> "numericarray-u16-id"
]

Test[
  LibraryFunctionLoad["liblibrary_tests", "test_na_f64_negate",
    {LibraryDataType[NumericArray, "Real64"]},
    LibraryDataType[NumericArray, "Real64"]][
    NumericArray[{1.0, -2.0, 3.5}, "Real64"]],
  NumericArray[{-1.0, 2.0, -3.5}, "Real64"]
  , TestID -> "numericarray-f64-negate"
]

(*==================================*)
(* Chrono date bridge (via WXF     *)
(* ByteArray).                      *)
(*==================================*)

With[{dateAddDays = LibraryFunctionLoad["liblibrary_tests", "test_date_add_days",
    {LibraryDataType[NumericArray, "UnsignedInteger8"], Integer},
    LibraryDataType[NumericArray, "UnsignedInteger8"]]},

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ dateAddDays[
      ByteArray @ Normal @ BinarySerialize[DateObject[{2026, 4, 14}]], 7],
    DateObject[{2026, 4, 21}]
    , TestID -> "chrono-naivedate-add-days"
  ];

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ dateAddDays[
      ByteArray @ Normal @ BinarySerialize[DateObject[{2026, 12, 31}]], 1],
    DateObject[{2027, 1, 1}]
    , TestID -> "chrono-naivedate-year-rollover"
  ]
]

With[{datetimeAddSeconds = LibraryFunctionLoad["liblibrary_tests", "test_datetime_add_seconds",
    {LibraryDataType[NumericArray, "UnsignedInteger8"], Integer},
    LibraryDataType[NumericArray, "UnsignedInteger8"]]},

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ datetimeAddSeconds[
      ByteArray @ Normal @ BinarySerialize[
        DateObject[{2026, 4, 14, 12, 30, 45}, "Instant", "Gregorian", "UTC"]],
      60],
    DateObject[{2026, 4, 14, 12, 31, 45}, "Instant", "Gregorian", "UTC"]
    , TestID -> "chrono-datetime-add-minute"
  ]
]

With[{buildDT = LibraryFunctionLoad["liblibrary_tests", "test_build_datetime",
    {Integer, Integer, Integer, Integer, Integer, Integer},
    LibraryDataType[NumericArray, "UnsignedInteger8"]]},

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ buildDT[2026, 4, 14, 12, 30, 45],
    DateObject[{2026, 4, 14, 12, 30, 45}, "Instant", "Gregorian", "UTC"]
    , TestID -> "chrono-build-datetime"
  ]
]

(*==================================*)
(* num_complex bridge                *)
(*==================================*)

With[{rot = LibraryFunctionLoad["liblibrary_tests", "test_complex_rotate_90",
    {LibraryDataType[NumericArray, "UnsignedInteger8"]},
    LibraryDataType[NumericArray, "UnsignedInteger8"]]},

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ rot[
      ByteArray @ Normal @ BinarySerialize[1.0 + 0.0 I]],
    Complex[0.0, 1.0]
    , TestID -> "num_complex-rotate-real"
  ];

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ rot[
      ByteArray @ Normal @ BinarySerialize[3.0 + 4.0 I]],
    Complex[-4.0, 3.0]
    , TestID -> "num_complex-rotate-3plus4i"
  ]
]

(*==================================*)
(* serde_json bridge                 *)
(*==================================*)

With[{jsonToWxf = LibraryFunctionLoad["liblibrary_tests", "test_json_to_wxf",
    {String}, LibraryDataType[NumericArray, "UnsignedInteger8"]]},

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ jsonToWxf["[1, 2, 3]"],
    {1, 2, 3}
    , TestID -> "json-bridge-array"
  ];

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ jsonToWxf["{\"a\": 1, \"b\": \"x\"}"],
    <| "a" -> 1, "b" -> "x" |>
    , TestID -> "json-bridge-object"
  ];

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ jsonToWxf["null"],
    Null
    , TestID -> "json-bridge-null"
  ];

  Test[
    BinaryDeserialize @ ByteArray @ Normal @ jsonToWxf["true"],
    True
    , TestID -> "json-bridge-true"
  ]
]

With[{wxfToJson = LibraryFunctionLoad["liblibrary_tests", "test_wxf_to_json",
    {LibraryDataType[NumericArray, "UnsignedInteger8"]}, String],
      serdeJson = LibraryFunctionLoad["liblibrary_tests", "test_expr_serde_json",
    {LibraryDataType[NumericArray, "UnsignedInteger8"]}, String]},

  Test[
    wxfToJson[ByteArray @ Normal @ BinarySerialize[{1, 2, 3}]],
    "[1,2,3]"
    , TestID -> "json-bridge-reverse-list"
  ];

  Test[
    wxfToJson[ByteArray @ Normal @ BinarySerialize[<|"a" -> 1, "b" -> "x"|>]],
    "{\"a\":1,\"b\":\"x\"}"
    , TestID -> "json-bridge-reverse-assoc"
  ];

  (* Native Serialize path: externally-tagged JSON, round-trip lossless. *)
  Test[
    StringContainsQ[
      serdeJson[ByteArray @ Normal @ BinarySerialize[42]],
      "\"integer\":42"],
    True
    , TestID -> "serde-native-integer-tag"
  ];

  Test[
    StringContainsQ[
      serdeJson[ByteArray @ Normal @ BinarySerialize[123456789012345678901234567890]],
      "\"bigInteger\":\"123456789012345678901234567890\""],
    True
    , TestID -> "serde-native-bigint-tag"
  ]
]
