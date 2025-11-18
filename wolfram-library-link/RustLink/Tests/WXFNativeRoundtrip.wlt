Needs["MUnit`"]

(* Load native test function that performs WL BinaryDeserialize/BinarySerialize roundtrip *)
wxfRoundtrip = LibraryFunctionLoad[
  "liblibrary_tests",
  "test_wxf_identity_roundtrip",
  {{LibraryDataType[NumericArray, "UnsignedInteger8"], "Constant"}},
  LibraryDataType[NumericArray, "UnsignedInteger8"]
];

roundtripThroughRust[expr_] := Module[{bytesBA, bytesNA, outNA, outBA, expr2},
  bytesBA = BinarySerialize[expr]; (* WL ByteArray *)
  bytesNA = NumericArray[Normal[bytesBA], "UnsignedInteger8"]; (* ByteArray -> NA[u8] *)
  outNA = wxfRoundtrip[bytesNA];
  outBA = ByteArray[Normal[outNA]]; (* NA[u8] -> ByteArray *)
  expr2 = BinaryDeserialize[outBA]; (* decode from ByteArray *)
  expr2
];

Test[roundtripThroughRust[42], 42, TestID -> "Integer"]
Test[roundtripThroughRust[3.14], 3.14, TestID -> "Real"]
Test[roundtripThroughRust["hello WXF"], "hello WXF", TestID -> "String"]
Test[roundtripThroughRust[Pi], Pi, TestID -> "Symbol"]
Test[roundtripThroughRust[{1,2,"a"}], {1,2,"a"}, TestID -> "List"]
Test[roundtripThroughRust[<|"a"->1, "b"->2.5|>], <|"a"->1, "b"->2.5|>, TestID -> "Association"]
Test[roundtripThroughRust[True], True, TestID -> "BooleanTrue"]
Test[roundtripThroughRust[False], False, TestID -> "BooleanFalse"]
Test[roundtripThroughRust[None], None, TestID -> "None"]
Test[roundtripThroughRust[1+2 I], 1+2 I, TestID -> "Complex"]

(* PackedArray *)
packed = Developer`ToPackedArray[{1,2,3,4}];
Test[Developer`PackedArrayQ[roundtripThroughRust[packed]], True, TestID -> "PackedArrayQ"]
Test[roundtripThroughRust[packed], {1,2,3,4}, TestID -> "PackedArrayContent"]

(* DateObject roundtrip *)
date = DateObject[{2025,11,14,12,30,0}];
Test[roundtripThroughRust[date], date, TestID -> "DateObject"]

(* BigInt (arbitrary precision integer) roundtrips *)
bigPos = 2^200 + 123456789; (* large positive integer *)
Test[roundtripThroughRust[bigPos], bigPos, TestID -> "BigIntPositive"]

bigNeg = -2^200 + 987654321; (* large negative integer *)
Test[roundtripThroughRust[bigNeg], bigNeg, TestID -> "BigIntNegative"]
