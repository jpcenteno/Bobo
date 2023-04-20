
# Bobo

Bobo is a lightweight HTTP server written in Rust for local previewing of static sites. It serves static files from a specified directory and includes LiveReload functionality.




## Usage/Examples

```javascript
RUST_LOG=info bobo --directory <DIRECTORY>
[2023-04-20T04:38:24Z INFO  bobo] Serving static files from directory: <DIRECTORY>
[2023-04-20T04:38:24Z INFO  bobo] Listening on 0.0.0.0:42069
```


## Documentation
```
bobo --help
Simple HTTP server for static files with live reload capabilities

Usage: bobo [OPTIONS] --directory <DIRECTORY>

Options:
  -d, --directory <DIRECTORY>  Path for the directory to serve
  -p, --port <PORT>            HTTP server port [default: 42069]
  -h, --help                   Print help
  -V, --version                Print version
```

Use the environment variable `RUST_LOG` to set the logging level. Refer to the [`env_logger` documentation](https://docs.rs/env_logger/latest/env_logger/#enabling-logging) to se the options.

## Running Tests

Yeah, sure. Just run the application and test it by hand. It has two features

```bash
RUST_LOG=debug  cargo run -- --directory <DIRECTORY>
```

