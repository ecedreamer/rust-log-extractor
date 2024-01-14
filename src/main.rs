mod extractor;


fn main() {
    let test_string_1 = "[INFO] User alice logged in from IP address 192.168.1.10.";
    let test_pattern_1 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    match extractor::extractor::parse_info(test_string_1, test_pattern_1) {
        Ok(matched) => println!("{:?}", matched),
        Err(error) => println!("Error: {}", error)
    }

    let test_string_2 = "2024-01-14 12:34:56 [INFO] User alice logged in from IP address 192.168.1.10.";
    let test_pattern_2 = "<log_time:date_time> [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    match extractor::extractor::parse_info(test_string_2, test_pattern_2) {
        Ok(matched) => println!("{:?}", matched),
        Err(error) => println!("Error: {}", error)
    }
}
