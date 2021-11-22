#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Build Paclet*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


(* ::Subsubsection:: *)
(*Copy LibraryResources libraries*)


$dir = FileNameJoin[{"RustLink", "LibraryResources", $SystemID}];


CreateDirectory[$dir]


Scan[
	exampleLib |-> CopyFile[
		FileNameJoin[{"target", "debug", "examples", exampleLib}],
		FileNameJoin[{$dir, exampleLib}]
	],
	{"libraw_wstp_function.dylib"}
]


(* ::Subsubsection:: *)
(*Build the paclet*)


result = PacletBuild[FileNameJoin[{Directory[], "RustLink"}]];

If[FailureQ[result],
	Echo @ CodeFormat @ ToString[result, InputForm];
	Exit[-1];
];
