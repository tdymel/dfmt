# Dynamic String Formatting

A fully featured dynamic drop in replacement for the format!, print!, println!, eprint!, eprintln!, write!, writeln! macro.

```rust
let str_template = "Black magic: {test:+^+#0width$.5b}";
let precompiled_template = Template::from(str_template);

println!(
    "Uses str template: {}", 
    dformat!(str_template, test = 42, width = 50)
);
println!(
    "Uses precompiled template: {}",
    dformat!(precompiled_template, test = 42, width = 50)
);
println!(
    "Uses format! macro under the hood: {}",
    dformat!("Black magic 4 {test:+^+#0width$.5b}", test = 42, width = 50)
);
// Outputs: "Black magic: +0b00000000000000000000000000000000000000000101010"

// TODO: Alterantive APIS
// TODO: I could also call it just "dformat"
// TODO: Add section about other libraries and why this one is better
```

## How it works
This crate uses the core::fmt machinery under the hood by design.
Therefore, you can expect the same behaviour, features and similar performance.
To provide such a pretty macro, some [black magic](https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html) was used.

## Safety
Returns: 
- Ok(str) - if your provided arguments match the type requirements of the patterns
- Err(..) - otherwise

## Performance
This crate uses the core::fmt machinery under the hood.
Therefore, you can expect the same performance compared to using **dynamic** arguments.
Obviously, we can't do const folding, as the compiler can.

### Overhead
* When creating the `Arguments` structure, a vector is allocated for the arguments. This is barely noticeable for many arguments.
* While the template parsing is fast, you can just create it once and then reuse it for multiple arguments.
* Right now padding a string with a fill character will cost some overhead.

### Nightly
If you are on nightly, you can opt in to the `nightly_formatting_options` feature to further improve the performance, 
especially for the fill character case and to reduce compilation complexity.

### Benchmarks
TODO

## License
This project is dual licensed under the Apache 2.0 license and the MIT license.