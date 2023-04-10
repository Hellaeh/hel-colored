# hel-colored
ANSI string colorizer and styler

## Features
* Lazy (no-alloc, no-copy) until needed, e.g. `to_string` called
* Made with performance in mind

## Examples
```rust
"Hello World!".bold().blue().underline();

let blue_text: ANSIString<&str> = "blue text".blue();
let green_blue_green_text: ANSIString<String> = format!("Green {blue_text} wrapping").green();
// Better to call `to_string` above
println!("{green_blue_green_text}");
```

## How to install
```
cargo add hel-colored
```
