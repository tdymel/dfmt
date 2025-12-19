# Contributing to Comfy I18n
🎉 First off, thanks for taking the time to contribute! We are so happy to have you! 🎉

**No contribution is too small and all contributions are valued.**

## How To Contribute
1. Create or pick an issue that outlines the problem that you would like to solve.
2. Create a fork of the repository and implement a solution.
3. Add tests for the new functionality.
4. If necessary, extend the documentation and the examples.
5. Create a pull a request and wait for our review. We will try to give you feedback as soon as we can.

## Architecture
The architecture resembles a pipeline of a transpiler.
1. Parsing: Parse the AST from various source formats
2. Validation: Missing translations, fmt arguments etc.
3. Optimization: Const folding of fmt functions
4. Generation: Generate translations for the desired target from the AST. 

### Misc crates:
* AST: Specifies the AST
* Locale/Context?!: Specification and ICU integration
* Macro: Crate that exposes the rust generator as a macro.
* Lib: Umbrella crate for the rust eco-system
