#!/usr/bin/env wolframscript -local /Applications/Wolfram/Mathematica-12.3.0.app/Contents/MacOS/WolframKernel
(* ::Package:: *)

(* ::Title:: *)
(*Run Tests*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


result = PacletTest[
	FileNameJoin[{Directory[], "RustLink"}],
	Parallelization -> False
];

result = Flatten[result];

If[!ListQ[result],
	Echo[CodeFormat @ ToString[result, InputForm], "not list"];
	Exit[-1];
];

Scan[
	Replace[report_ :> Module[{failedCases},
		If[Head[report] =!= TestReportObject,
			Echo[CodeFormat @ ToString[result, InputForm], "unexpected head"];
			Exit[-1];
		];

		failedCases = Flatten[Values[Join[Values[report["TestsFailed"]]]]];

		Replace[failedCases, {
			{__} :> (
				Print[];
				Do[
					Print["=========================="];
					Print["Failed Case: \n"];
					Print[CodeFormat @ ToString[case, InputForm]];
					Print["\n"];
					,
					{case, failedCases}
				];
			),
			{} :> Null,
			_ :> (
				Exit[-1]
			)
		}]
	]]
	,
	result
]
