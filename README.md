# BLC and BCL

Interpreter for [Binary Lambda Calculus](https://esolangs.org/wiki/Binary_lambda_calculus) and [Binary Combinatory Logic](https://esolangs.org/wiki/Binary_combinatory_logic)

## To-do

* Lambda Calculus (LC)
  * LC parser
  * LC to BLC transpiler
  * BLC parser
  * BLC to LC transpiler
* Combinatory Logic (CL)
  * CL parser
  * CL to BCL transpiler
  * BCL parser
  * BCL to CL transpiler
* Between LC and CL
  * Optimizing LC to CL transpiler ([Reference](https://tromp.github.io/cl/LC.pdf))
  * CL to LC transpiler
* User language (to be named later) (for convenience, allows mixing LC and CL, and auxiliary definitions)
  * Grammar definition
  * Parser
  * To-LC compiler
  * Optimizing to-CL compiler (e.g. beta-reduce or not before conversion)
* Interpreter interface
  * Custom I/O definition (Church numerals, booleans, lists)
  * Input and output transformer
  * LC/CL interpreter (which mode to run in is TBD)
  * CLI
* Online interpreter page
  * Interface structs and functions for Wasm target
  * UI (textarea, converter buttons, input and output area)
  * Byte count scorer (LC, BLC, CL, BCL) for CGCC submission
  * Permalink
