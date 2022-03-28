#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Build Paclet*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]

(*----------------------------------------------------------*)
(* Verify that Directory[] is the repository root directory *)
(*----------------------------------------------------------*)

$repositoryDir = Directory[];

If[FileNameTake[$repositoryDir] =!= "wolfram-library-link-rs",
	Throw[StringForm["Unexpected repository root directory path: ``", $repositoryDir]]
];

(*-------------------------------------------------------------*)
(* Build the RustLink paclet into $repositoryDir/build/ *)
(*-------------------------------------------------------------*)

(* ::Subsubsection:: *)
(*Build the paclet*)


result = PacletBuild[
	FileNameJoin[{$repositoryDir, "wolfram-library-link", "RustLink"}],
	FileNameJoin[{$repositoryDir, "build"}]
];

If[FailureQ[result],
	Echo @ CodeFormat @ ToString[result, InputForm];
	Exit[-1];
];

(*-----------------------------------------------------------------------------------*)
(* Copy the examples and tests dynamic libraries into the LibraryResources directory *)
(*-----------------------------------------------------------------------------------*)

(* ::Subsubsection:: *)
(*Copy LibraryResources libraries*)


$dir = FileNameJoin[{$repositoryDir, "build", "RustLink", "LibraryResources", $SystemID}];


CreateDirectory[$dir]


Scan[
	exampleLib |-> CopyFile[
		FileNameJoin[{"target", "debug", "examples", exampleLib}],
		FileNameJoin[{$dir, exampleLib}]
	],
	{
		(* Library Tests *)
		"liblibrary_tests.dylib",
		(* Examples *)
		"libbasic_types.dylib",
		"libraw_wstp_function.dylib",
		"libbasic_expressions.dylib",
		"libmanaged_exprs.dylib",
		"libnumeric_arrays.dylib",
		"libraw_librarylink_function.dylib",
		"libasync_file_watcher_raw.dylib",
		"libasync_file_watcher.dylib",
		"libdata_store.dylib",
		"libaborts.dylib",
		"libwstp_example.dylib",
		"libwll_docs.dylib"
	}
]
