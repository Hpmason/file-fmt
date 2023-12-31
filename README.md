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
