# hel-colored

ANSI string colorizer and styler

## Features

- Lazy (no-alloc, no-copy, until needed, e.g. `to_string` called)
- Made with performance in mind
- Only RGB support (no default CLI colors)
- As of now there's no way to opt-out of nesting check, which should lead to significant performance boost

## Examples

### Simply by

```rust
"Hello World!".bold().blue().underline();
```

### Or nested

```rust
let blue_text: ANSIString<&str> = "blue text".blue();
let green_blue_green_text: ANSIString<String> = format!("Green {blue_text} wrapping").green();
// Better to call `to_string` above
println!("{green_blue_green_text}");
```

### Or with builder

```rust
let builder: ANSIStringBuilder = ANSIStringBuilder::new().bold().underline().orange();
let str1 = builder.build("Hello");
let str2 = builder.build("World!");
println!("{str1}, {str2}")
```

## How to install

```
cargo add hel-colored
```
