## LOG EXTRACTOR USING USER DEFINED PATTERN: WRITTEN IN RUST ##

The Log Extractor is a Rust project that provides a module (extractor) for parsing log messages using custom patterns. It allows you to define patterns for log messages and extract relevant information such as log level, timestamp, username, and IP address.

### Clone the project and go in to the project directory. ####

```sh
projects % git clone https://github.com/ecedreamer/rust-log-extractor.git
projects % cd custom_pattern_matcher
```
#### To run it ####
```sh
custom_pattern_matcher % ./run.sh

# or 

custom_pattern_matcher % RUST_LOG=info cargo run --release

```


### Defining Patterns
The Log Extractor uses custom patterns to define the structure of log messages. The general pattern is `<key_name:data_type>`.Since this is just for the experiment, only couple of datatypes are available. You can specify placeholders like `<log_level:word>`, `<username:word>`, `<ip_address:ip>`, and `<log_time:date_time>`, and the extractor will attempt to match and extract information accordingly.

### Examples
#### test Case 1
```rust
let test_string_1 = "[INFO] User alice logged in from IP address 192.168.1.10.";
let test_pattern_1 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
```

#### Output
```rust
[Matched { key: "log_level", value: "INFO" }, Matched { key: "username", value: "alice" }, Matched { key: "ip_address", value: "192.168.1.10" }]
```

#### test Case 2
```rust
let test_string_2 = "2024-01-14 12:34:56 [INFO] User alice logged in from IP address 192.168.1.10.";
let test_pattern_2 = "<log_time:date_time> [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";

```

#### Output
```rust
[Matched { key: "log_time", value: "2024-01-14 12:34:56" }, Matched { key: "log_level", value: "INFO" }, Matched { key: "username", value: "alice" }, Matched { key: "ip_address", value: "192.168.1.10" }]
```

#### Note: This is just for experimental purpose not the production ready. One should optimize this and handle errors to use for production. 

