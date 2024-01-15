## LOG EXTRACTOR USING USER DEFINED PATTERN: WRITTEN IN RUST ##

The Log Extractor is a Rust project that provides a module (extractor) for parsing log messages using custom patterns. It allows you to define patterns for log messages and extract relevant information such as log level, timestamp, username, and IP address.

### Clone the project and go in to the project directory. ####

```sh
projects % git clone https://github.com/ecedreamer/rust_log_extractor.git
projects % cd rust_log_extractor
```
#### To run it ####
```sh
rust_log_extractor % ./run.sh

# or 

rust_log_extractor % RUST_LOG=info cargo run --release

```


### Defining Patterns
The Log Extractor uses custom patterns to define the structure of log messages. The general pattern is `<key_name:data_type>`.Since this is just for the experiment, only couple of datatypes are available. You can specify placeholders like `<log_level:word>`, `<username:word>`, `<ip_address:ip>`, and `<log_time:date_time>`, and the extractor will attempt to match and extract information accordingly.


### OUTPUT
```log
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Starting main...
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test String 1: [INFO] User alice logged in from IP address 192.168.1.10.
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test Pattern 1: [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.
[2024-01-15T07:49:12Z INFO  rust_log_extractor] [Matched { key: "log_level", value: "INFO" }, Matched { key: "username", value: "alice" }, Matched { key: "ip_address", value: "192.168.1.10" }]
    
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test String 2: 2024-01-14 12:34:56 [INFO] User alice logged in from IP address 192.168.1.10.
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test Pattern 2: <log_time:date_time> [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.
[2024-01-15T07:49:12Z INFO  rust_log_extractor] [Matched { key: "log_time", value: "2024-01-14 12:34:56" }, Matched { key: "log_level", value: "INFO" }, Matched { key: "username", value: "alice" }, Matched { key: "ip_address", value: "192.168.1.10" }]
    
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test String 3: [INFO] User dipak logged in from IP address 136.24.10.44
[2024-01-15T07:49:12Z INFO  rust_log_extractor] Test Pattern 3: [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.
[2024-01-15T07:49:12Z INFO  rust_log_extractor] [Matched { key: "log_level", value: "INFO" }, Matched { key: "username", value: "dipak" }, Matched { key: "ip_address", value: "136.24.10" }]
```

#### Note: This is just for experimental purpose not the production ready. One should optimize this and handle errors to use for production. 

