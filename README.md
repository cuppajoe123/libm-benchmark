# musl libm benchmark

This repository transforms the math library libm into a benchmark for testing C->Rust translations.

libm is standard C math library. The implementation we use is from [musl](https://github.com/bminor/musl). This implementation has been rewritten in [Rust](https://github.com/rust-lang/libm).

To test a code translation technique, the Python script can be executed with a customized prompt and/or source file. It translates the specified C code to Rust using an LLM and appends the output to the corresponding source file in the Rust libm codebase. It then attempts to compile and test the codebase to verify correctness.

libm was chosen as the basis for this benchmark because math functions are well defined and often do not have complex dependencies. Additionally, libm will require minimal effort in order to fully utilize the codebase for testing.

## Analysis of libm codebase and other findings
- 107 functions are implemented in both C and Rust
- There is only one header that some of the C functions depend on, called libm.h
- libm.h just contains some simple helper functions and macros
- 77 C files do not depend on libm.h and can therefore be translated with no extra preprocessing
- Of these 77 C functions, 35 are implemented in Rust
- Of these 35 functions, 8 have unit tests already written. These 8 functions can be used for benchmarking without any additional work.
- ChatGPT can effectively generate Rust unit tests for these well-defined math functions
- Since libm.h defines simple functions and macros, it can either be translated to Rust as its own module, or each definition a file needs can be preprocessed in before translation.
