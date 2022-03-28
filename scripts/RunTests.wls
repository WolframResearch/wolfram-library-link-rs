#!/usr/bin/env wolframscript -local /Applications/Wolfram/Mathematica-13.0.0.app/Contents/MacOS/WolframKernel
(* ::Package:: *)

(* ::Title:: *)
(*Run Tests*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]

(*----------------------------------------------------------*)
(* Verify that Directory[] is the repository root directory *)
(*----------------------------------------------------------*)

$repositoryDir = Directory[];

If[FileNameTake[$repositoryDir] =!= "wolfram-library-link-rs",
	Throw[StringForm["Unexpected repository root directory path: ``", $repositoryDir]]
];

(*---------------*)
(* Run the tests *)
(*---------------*)

result = PacletTest[
	FileNameJoin[{$repositoryDir, "build", "RustLink"}],
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
