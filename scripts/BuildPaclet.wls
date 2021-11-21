#!/usr/bin/env wolframscript
(* ::Package:: *)

(* ::Title:: *)
(*Build Paclet*)


Needs["PacletTools`"]
Needs["CodeFormatter`"]


Echo @ CodeFormat @ ToString[PacletBuild[Directory[]], InputForm]
