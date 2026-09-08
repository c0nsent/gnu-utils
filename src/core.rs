use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgType {
    Int,
    Text,
    None
}
pub struct Options {

    long_options: HashMap<String, ArgType>,
    short_options: HashMap<char, ArgType>
}

impl Options {

    fn parse_long_option(option: &str) -> Option<String> {
        let option = option.strip_prefix("--")?;

        let first = option.chars().next()?;
        let last = option.chars().last()?;

        if !first.is_alphanumeric() || !last.is_alphanumeric() {
            return None
        }

        option
            .chars()
            .all(|ch| ch.is_alphanumeric() || ch == '-')
            .then(|| option.to_string())
    }

    fn parse_short_option(option: &str) -> Option<char> {
        let mut option = option.chars();

        if option.next_back()?.eq(&'-')
            && option.next()?.is_alphanumeric()
            && option.next().is_none() {

            return Some(option.nth(0)?)
        }

        None
    }
    pub fn new() -> Self {
        Options {
            long_options: HashMap::new(),
            short_options: HashMap::new()
        }
    }

    pub fn add_option(&mut self, option: &str, argument_type: ArgType) -> Result<(), String>{

        //TODO: Почему-то это условие всегда выдает тру
        if option.starts_with("--") {
            let long = Self::parse_long_option(option)
                .ok_or_else(|| format!("{} is not a valid long option", option))?;

            self.long_options.insert(long, argument_type);
            Ok(())
        }
        else if option.starts_with("-") {
            let short = Self::parse_short_option(option)
                .ok_or_else(|| format!("{} is not a valid long option", option))?;

            self.short_options.insert(short, argument_type);
            Ok(())
        }
        else {
            Err(format!("{} doesn't have a valid prefix", option))
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        option.strip_prefix("--").is_some_and(|result| self.long_options.contains_key(result))
            || Self::parse_short_option(option)
            .is_some_and(|result| self.short_options.contains_key(&result))
    }

    pub fn get_type(&self, option: &str) -> Option<&ArgType> {
        if let Some(long) = option.strip_prefix("--") {
            self.long_options.get(long)
        }
        else if let Some(short) = Self::parse_short_option(option) {
            self.short_options.get(&short)
        }
        else {
            None
        }
    }
}



pub struct Args {
    args: HashMap<String, String>,
    last_value: Option<String>
}

impl Args {
/*    fn parse(viable_options: Options) -> Result<Self, String> {
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
                    ArgType::String => String::new(&value).exists(),
                    ArgType::Int => value.parse::<i32>().is_ok(),
                    ArgType::Uint => value.parse::<u32>().is_ok(),
                    ArgType::NoValue => true
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
    }*/

    fn parse() {

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