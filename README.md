# file-fmt

Simple file formatter for data files for minifying and prettying files.

Supports json + toml (two main serde formats that support pretty output), but
can be easily modified to for other serde formats. There are faster ways of 
reading/writing files like json, but this is still quick and simple.

**NOTE: Removes comments and can modify ordering. Not recommended if you want 
to perserve those things**

# Install
Install from git with
```
cargo install --git https://github.com/Hpmason/file-fmt.git
```

Install locally with
```
cargo install --path .
```

# Usage
Minify file
```
file-fmt minify data.json
```

Pretty file
```
file-fmt pretty data.json
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
