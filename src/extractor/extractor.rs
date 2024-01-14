use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Matched {
    pub key: String,
    pub value: String
}

fn get_key_to_regex_map<'a>() -> HashMap<&'a str, &'a str> {
    let mut key_to_regex_map = HashMap::new();
    key_to_regex_map.insert("word", r"\w+");
    key_to_regex_map.insert("int", r"[+-]?\ *\d+");
    key_to_regex_map.insert("ip", r"\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}");
    key_to_regex_map.insert("port", r"\d{1,5}");
    key_to_regex_map.insert("words", r"(?:\w+\s?)+");
    key_to_regex_map.insert("date_time", r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}");
    key_to_regex_map
}

fn get_keys_info(pattern: &str) -> HashMap<String, String> {
    let mut key_list = HashMap::new();
    let mut key = String::new();
    let mut key_started = false;
    for ch in pattern.chars() {
        match ch {
            '<' => key_started = true,
            '>' if key_started => {
                key_started = false;
                let splitted: Vec<&str> = key.split(":").collect();
                key_list.insert(splitted[0].to_string(), splitted[1].to_string());
                key.clear();
            }
            _ if key_started => key.push(ch),
            _ => {}
        }
    }
    key_list
}

fn convert_pattern_to_regex(pattern: &str) -> String{
    let keys_info = get_keys_info(&pattern);

    let mut final_string = pattern.to_string();
    final_string = regex::escape(&final_string);

    let key_to_regex_map = get_key_to_regex_map();
    for (key, value) in keys_info.iter() {
        final_string = final_string.replace(
            format!("<{key}:{value}>").as_str(),
            format!("(?P<{}>{})", key, key_to_regex_map.get(value.as_str()).unwrap()).as_str()
        );
    }
    format!(r"{final_string}")

}


pub fn parse_info(input_str: &str, pattern: &str) -> Result<Vec<Matched>, String> {
    let mut parsed_info: Vec<Matched> = vec![];
    let regex_pattern = convert_pattern_to_regex(pattern);

    let compiled_pattern = regex::Regex::new(&regex_pattern).expect("Could not convert pattern");

    if let Some(captures) = compiled_pattern.captures(input_str) {
        for name in compiled_pattern.capture_names().filter_map(|n| n) {
            if let Some(matched) = captures.name(name) {
                parsed_info.push(Matched{key: name.to_string(), value: matched.as_str().to_string()});
            }
        }
    Ok(parsed_info)

    } else {
        Err("Could not parse infomation".to_string())
    }
}