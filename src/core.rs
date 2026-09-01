use std::collections::HashMap;
use std::env;
use crate::cli_options::{OptionType, Options};
use std::path::Path;


pub struct Args {
    args: HashMap<String, String>,
    last_value: Option<String>
}

impl Args {
    fn parse(viable_options: Options) -> Result<Self, String> {
        let mut raw_args = env::args();

        let last_value = raw_args.next_back().unwrap();

        let mut processed_args = HashMap::new();

        for mut i in 1..raw_args.len() - 1 {

            let flag = raw_args.nth(i).unwrap();
            i+= 1;
            let value = raw_args.nth(i).unwrap();

            if  viable_options.has_option(&flag) {

                let option_type = viable_options.get_option_type(&flag)?;

                let is_viable = match option_type {
                    OptionType::Path => Path::new(&value).exists(),
                    OptionType::Int => value.parse::<i32>().is_ok(),
                    OptionType::Uint => value.parse::<u32>().is_ok(),
                    OptionType::NoValue => true
                };

                if (is_viable) {
                    processed_args.insert(flag, value);
                }
                else {
                    return Err( format!("Parsed value is not viable: {}", value));
                }
            }


        };

        Ok(Self{
            args: processed_args,
            last_value: Some(last_value)
        })
    }

    fn contains(&self, flag: &str) -> bool {
        self.args.contains_key(flag)
    }

    fn at(&self, flag: &str) -> Option<String> {
        self.args.get(flag).cloned()
    }

    fn has_last_value(&self) -> bool {
        self.last_value.is_some()
    }

    fn last_value(&self) -> Option<String> {
        self.last_value.clone()
    }
}