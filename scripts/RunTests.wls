#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Run Tests*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


result = PacletTest[
	FileNameJoin[{Directory[], "RustLink"}],
	Parallelization -> False
];


Echo @ CodeFormat @ ToString[result, InputForm];
