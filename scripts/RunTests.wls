#!/usr/bin/env wolframscript -local /Applications/Wolfram/Mathematica-13.1.0.app/Contents/MacOS/WolframKernel
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

codeFormattedString[expr_] :=
	CodeFormat[
		ToString[expr, InputForm],
		"BreakLinesMethod" -> "LineBreakerV2"
	]

Scan[
	Replace[report_ :> Module[{failedCases},
		If[Head[report] =!= TestReportObject,
			Echo[codeFormattedString[result], "unexpected head"];
			Exit[-1];
		];

		failedCases = Flatten[Values[Join[Values[report["TestsFailed"]]]]];

		Replace[failedCases, {
			{__} :> (
				Print[];
				Do[
					Print["=========================="];
					Print["Failed Case: \n"];
					Print[codeFormattedString[case]];
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
