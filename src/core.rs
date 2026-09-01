use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use crate::cli_options::{OptionType, Options};
use std::path::Path;


pub struct Args {
    args: HashMap<String, String>,
    last_value: String
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

                let optionType = viable_options.get_option_type(&flag);

                match optionType {
                    OptionType::Path => Path::new()
                };

            }
        };

        /* `core::Args` value */
    }
}