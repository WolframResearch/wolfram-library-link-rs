#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Build Paclet*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


result = PacletBuild[FileNameJoin[{Directory[], "RustLink"}]];

Echo @ CodeFormat @ ToString[result, InputForm];

If[FailureQ[result],
	Exit[-1];
];
