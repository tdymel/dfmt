# dfmt - `d`ynamic `format!`

`dfmt` provides `core::fmt`-like formatting for **dynamic templates** and is a **fully featured** dynamic drop in replacment for the macros: `format!`, `print!`, `println!`, `eprint!`, `eprintln!`, `write!`, `writeln!`.

```rust
// Check out the documentation for a complete overview.
use dfmt::*;

let str_template = "Hello, {0} {{{world}}} {} {day:y<width$}!";
let precompiled_template = Template::parse(str_template).unwrap();

// Parsing the str template on the fly
dprintln!(str_template, "what a nice", world = "world", day = "day", width=20);

// Using a precompiled template
dprintln!(precompiled_template, "what a nice", world = "world", day = "day", width=20);

// Uses println! under the hood
dprintln!("Hello, {0} {{{world}}} {} {day:y<width$}!", "what a nice", 
    world = "world", day = "day", width=20);

// Other APIs
let using_dformat = dformat!(precompiled_template, "what a nice", 
    world = "world", day = "day", width=20).unwrap();
println!("{}", using_dformat);

let using_manual_builder_api = precompiled_template
    .arguments()
    .builder()
    .display(0, &"what a nice")
    .display("world", &"world")
    .display("day", &"day")
    .width_or_precision_amount("width", &20)
    .format()
    .unwrap();
println!("{}", using_manual_builder_api);

let using_str_extension = "Hello, {0} {{{world}}} {} {day:y<width$}!"
    .format(vec![
        (
            ArgumentKey::Index(0),
            ArgumentValue::Display(&"what a nice"),
        ),
        (
            ArgumentKey::Name("world".to_string()),
            ArgumentValue::Display(&"world"),
        ),
        (
            ArgumentKey::Name("day".to_string()),
            ArgumentValue::Display(&"day"),
        ),
        (
            ArgumentKey::Name("width".to_string()),
            ArgumentValue::WidthOrPrecisionAmount(&20),
        ),
    ])
    .unwrap();
println!("{}", using_str_extension);

let using_manual_template_builder = Template::new()
    .literal("Hello, ")
    .specified_argument(0, Specifier::default()
        .alignment(Alignment::Center)
        .width(Width::Fixed(20)))
    .literal("!")
    .arguments()
    .builder()
    .display(0, &"World")
    .format()
    .unwrap();
println!("{}", using_manual_template_builder);
```

## Features
✅ **Support dynamic templates**  
✅ **All formatting specifiers**  
✅ **Indexed and named arguments**  
✅ **Easy to use API and macros**  
✅ **With safety in mind**  
✅ **Blazingly fast**  
✅ **No-std support (Using a global allocator, and only dformat! and write!)**

### Formatting features
| Name | Feature |
| ---- | ------- |
| Fill/Alignment | `<`, `^`, `>` |
| Sign | `+`, `-` |
| Alternate | `#` |
| Zero-padding | `0` |
| Width | `{:20}`, `{:width$}` |
| Precision | `{:.5}`, `{:.precision$}`, `{:*}` |
| Type | `?`, `x`, `X`, `o`, `b`, `e`, `E`, `p` |
| Argument keys | `{}`, `{0}`, `{arg}` |

## How it works
* If the template is a literal, then the `format!` macro is used under the hood.
* Uses the `core::fmt` machinery under the hood. Therefore, you can expect the same formatting behaviour.
* It uses [black magic](https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html) to provide a comfortable macro.

## Safety
There are multiple runtime checks to prevent you from creating an invalid format string.
* Check if the required argument value exists and implements the right formatter.
* Check for duplicate arguments
* Validate the template

## Performance
In the best case `dfmt` is as fast as `format!`. In the worst case, its up to 60% - 100% slower.

However, I believe with further optimization this gap could be closed. In fact, with the `formatting_options` feature we are even faster in some cases.

### Considerations
* While the template parsing is fast, you can just **create it once and then reuse it** for multiple arguments.
* There is a **unchecked** version, which skips safety checks.
* If the template is a literal, it will fall back to **format!** internally if you use the macro.

### Overhead
* When creating the `Arguments` structure, a vector is allocated for the arguments. This is barely noticeable for many arguments.
* Right now padding a string with a fill character will cost some overhead.
* If a pattern reuses an argument multiple times, it will push a typed version of this value multiple times right now. This allocates more memory, but is required to provide a convinient API.

### Nightly
If you are on nightly, you can opt in to the `nightly_formatting_options` feature to further improve the performance, 
especially for the fill character case and to reduce compilation complexity.

### Benchmarks
These benchmarks compare `dfmt` with `format!` with dynamic arguments only. Obviously, if `format!` makes use of const folding, it will be much faster.

#### Without `formatting_options` feature

| Benchmark | simple - 1 arg | simple - 7 args | complex |
| --------- | -------------- | --------------- | ------- |
| Template::parse | 67 ns | 249 ns | 684 ns |
| **format!** | **30 ns** | 174 ns | **515 ns** |
| Template unchecked | 46 ns | **173 ns** | 845 ns |
| Template checked | 49 ns | 250 ns | 911 ns |
| dformat! unchecked | 51 ns | 235 ns | 952 ns |
| dformat! checked | 51 ns | 260 ns | 1040 ns |

#### With `formatting_options` feature
| Benchmark | simple - 1 arg | simple - 7 args | complex |
| --------- | -------------- | --------------- | ------- |
| Template::parse | 67 ns | 249 ns | 684 ns |
| **format!** | **30 ns** | 174 ns | 515 ns |
| Template unchecked | 46 ns | **169 ns** | **464 ns** |
| Template checked | 49 ns | 238 ns | 527 ns |
| dformat! unchecked | 51 ns | 232 ns | 576 ns |
| dformat! checked | 51 ns | 257 ns | 658 ns |

## Minimal rustc version
Right now it compiles until 1.81, this is when `std::error` went into `core::Error`. 
You can opt out of `error`-impl by disabling the feature `error`. Then you can go down until 1.56.

## License
This project is dual licensed under the Apache 2.0 license and the MIT license.