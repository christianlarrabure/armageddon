use super::super::settings::get_config;
use serde_json::{Map, Value};

fn get_aliases() -> Map<String, Value> {
    let aliases = get_config("aliases".to_string());
    let aliases = aliases.as_object().unwrap();
    aliases.clone()
}

pub fn transform_with_aliases(input: &str) -> String {
    let aliases = get_aliases();
    let parsed_input: Vec<&str> = input.split(' ').collect();
    let parsed_input_length = parsed_input.len();

    let verb = parsed_input[0];
    let args = &parsed_input[1..parsed_input_length];

    for alias in aliases.iter() {
        if !verb.eq(alias.0.as_str()) {
            continue;
        }

        // we have found the alias.

        let mut input = String::from(format!("{}", alias.1.as_str().unwrap()));
        let mut count: i32 = 0;

        for arg in args.iter() {
            count = count + 1;
            let arg = arg.to_string();
            input = input.replace(&format!("${}", count.to_string()), &arg);
        }
        return input;
    }

    input.to_string()

    /*     let replacement = aliases[input].clone();

       if replacement == Value::Null {
           return input.to_string();
       }

       let new_input = format!("{}", replacement.as_str().unwrap());
       new_input
    */
}
