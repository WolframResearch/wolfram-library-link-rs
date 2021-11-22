#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Build Paclet*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


result = PacletBuild[FileNameJoin[{Directory[], "RustLink"}]];

If[FailureQ[result],
	Echo @ CodeFormat @ ToString[result, InputForm];
	Exit[-1];
];
