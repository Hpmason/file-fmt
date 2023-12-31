# file-fmt

Simple file formatter for data files for minifying and prettying files.

Supports json + toml (two main serde formats that support pretty output), but
can be easily modified to for other serde formats.

**NOTE: Removes comments and can modify ordering. Not recommended if you want 
to perserve those things**

