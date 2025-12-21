# dft - A dynamic core::fmt format! drop in replacement 

Dfmt is a fully featured dynamic drop in replacement for the `format!`, `print!`, `println!`, `eprint!`, `eprintln!`, `write!`, `writeln!` macros.

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
This crate uses the `core::fmt` machinery under the hood.
Therefore, you can expect the similar performance compared to using **dynamic** arguments.
Obviously, we can't do const folding, as the compiler can.

### Overhead
* When creating the `Arguments` structure, a vector is allocated for the arguments. This is barely noticeable for many arguments.
* While the template parsing is fast, you can just **create it once and then reuse it** for multiple arguments.
* Right now padding a string with a fill character will cost some overhead.
* If a pattern reuses an argument multiple times, we will push a typed version of this value multiple times right now. This allocates more memory, but is required to provide a convinient API.

### Nightly
If you are on nightly, you can opt in to the `nightly_formatting_options` feature to further improve the performance, 
especially for the fill character case and to reduce compilation complexity.

### Benchmarks
`dfmt` is not as fast as `format!` yet, but I think it should be possible to be at least as fast as the `format!` equivalent with dynamic args with further optimization.

TODO: Order by whats fastest and then show % change


| Bechmark | Runtime performance |
| -------- | ------------------- |
| Template::parse - simple - 1 arg | 69 ns |
| Template::parse - simple - 7 arg | 292 ns |
| Template::parse - complex | 693 ns |
| dformat! - simple - 1 arg | 51 ns |
| dformat_unchecked! - simple - 1 arg | 51 ns |
| format! - simple - 1 arg | 30 ns |
| Manual via template - simple - 1 arg | 49 ns |
| Manual via template unchecked - simple - 1 arg | 46 ns |
| dformat! - simple - 7 args | 260 ns |
| dformat_unchecked! - simple - 7 args | 235 ns |
| format! - simple - 7 args | 174 ns |
| Manual via template - simple - 7 args | 250 ns |
| Manual via template unchecked - simple - 7 args | 173 ns |
| dformat! - complex | 1040 ns |
| dformat_unchecked! - complex | 952 ns |
| format! - complex | 520 ns |
| Manual via template - complex | 911 ns |
| Manual via template unchecked - complex | 845 ns |

## License
This project is dual licensed under the Apache 2.0 license and the MIT license.