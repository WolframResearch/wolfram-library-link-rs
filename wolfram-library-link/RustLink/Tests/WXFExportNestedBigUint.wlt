Needs["MUnit`"]

nestedExport = LibraryFunctionLoad[
  "liblibrary_tests",
  "test_wxf_export_nested_biguint",
  {},
  LibraryDataType[NumericArray, "UnsignedInteger8"]
];

bytesNA = nestedExport[];
bytesBA = ByteArray[Normal[bytesNA]];
expr = BinaryDeserialize[bytesBA];

(* Expected WL structure: {{ {42, big}, Null }, { Null, {7, 99} }} with big matching the large integer value *)
big = 123456789012345678901234567890; (* same decimal literal used in Rust *)
expected = {{ {42, big}, Null }, { Null, {7, 99} }};

Test[expr, expected, TestID -> "NestedBigUintStructure"]
Test[expr[[1,1,2]], big, TestID -> "BigUintLargeValue"]
Test[IntegerQ[expr[[1,1,2]]], True, TestID -> "BigUintIsInteger"]
Test[expr[[2,2,2]], 99, TestID -> "SmallBigUintValue"]
