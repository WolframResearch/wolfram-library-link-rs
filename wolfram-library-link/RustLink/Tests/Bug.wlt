Needs["MUnit`"]


(* DataStore *)

Test[
	ByteArray @ LibraryFunctionLoad["liblibrary_tests", "test_string_vec",
        {"DataStore"}, LibraryDataType[NumericArray, "UnsignedInteger8"]
    ][Developer`DataStore[]]
	,
	BinarySerialize[{{None}}]
]
