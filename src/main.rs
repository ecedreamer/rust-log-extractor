use log::{info, error};
use std::time::Instant;
mod extractor;


fn main() {

    env_logger::init();
    info!("Starting main...");

    // test sample 1
    let test_string_1 = "[INFO] User alice logged in from IP address 192.168.1.10.";
    let test_pattern_1 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    match extractor::extractor::parse_info(test_string_1, test_pattern_1) {
        Ok(matched) => {
            info!("Test String 1: {test_string_1}");
            info!("Test Pattern 1: {test_pattern_1}");
            info!("{:?}\n", matched);
        },
        Err(error) => error!("Error: {}\n", error)
    }

    // test sample 2
    let test_string_2 = "2024-01-14 12:34:56 [INFO] User alice logged in from IP address 192.168.1.10.";
    let test_pattern_2 = "<log_time:date_time> [<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    match extractor::extractor::parse_info(test_string_2, test_pattern_2) {
        Ok(matched) => {
            info!("Test String 2: {test_string_2}");
            info!("Test Pattern 2: {test_pattern_2}");
            info!("{:?}\n", matched);
        },
        Err(error) => error!("Error: {}\n", error)
    }

    // test sample 3
    let test_string_3 = "[INFO] User dipak logged in from IP address 136.24.10.44";
    let test_pattern_3 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
    match extractor::extractor::parse_info(test_string_3, test_pattern_3) {
        Ok(matched) => {
            info!("Test String 3: {test_string_3}");
            info!("Test Pattern 3: {test_pattern_3}");
            info!("{:?}\n", matched);
        },
        Err(error) => error!("Error: {}\n", error)
    }

    // testing performance of 
    let start_time = Instant::now();
    let mut success_result = 0;
    let mut fail_result = 0;
    for _ in 1..10001 {
        let test_string_3 = "[INFO] User dipak logged in from IP address 136.24.10.44";
        let test_pattern_3 = "[<log_level:word>] User <username:word> logged in from IP address <ip_address:ip>.";
        match extractor::extractor::parse_info(test_string_3, test_pattern_3) {
            Ok(_) => success_result += 1,
            Err(_) => fail_result += 1
        }
    }
    let elapsed_time = start_time.elapsed();
    println!("Success Result: {success_result}; Fail Result: {fail_result}, {elapsed_time:?}");
    
}
