Needs["MUnit`"]

(* Test DataStore-based native loader *)
Test[
	loader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_native", {"UTF8String"}, "DataStore"];
	meta = loader["libbasic_types"];
	Head[meta]
,
	Developer`DataStore
]

(* Test that DataStore contains expected function entries *)
Test[
	loader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_native", {"UTF8String"}, "DataStore"];
	meta = loader["libbasic_types"];
	rules = Normal[meta];
	MemberQ[rules, "square" -> _]
,
	True
]

(* Test WXF-based loader *)
Test[
	wxfLoader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_wxf", {"UTF8String"}, 
		LibraryDataType[NumericArray, "UnsignedInteger8"]];
	wxfBytes = wxfLoader["libbasic_types"];
	assoc = BinaryDeserialize[ByteArray[wxfBytes]];
	AssociationQ[assoc]
,
	True
]

(* Test that WXF loader returns valid LibraryFunction expressions *)
Test[
	wxfLoader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_wxf", {"UTF8String"}, 
		LibraryDataType[NumericArray, "UnsignedInteger8"]];
	wxfBytes = wxfLoader["libbasic_types"];
	assoc = BinaryDeserialize[ByteArray[wxfBytes]];
	KeyExistsQ[assoc, "square"] && MatchQ[assoc["square"], _LibraryFunction]
,
	True
]

(* Test that WXF LibraryFunction actually works *)
Test[
	wxfLoader = LibraryFunctionLoad["libbasic_types", "load_basic_types_functions_wxf", {"UTF8String"}, 
		LibraryDataType[NumericArray, "UnsignedInteger8"]];
	wxfBytes = wxfLoader["libbasic_types"];
	assoc = BinaryDeserialize[ByteArray[wxfBytes]];
	assoc["square"][5]
,
	25
]

