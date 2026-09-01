use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionType {
    Path,
    Uint,
    Int,
    NoValue,
}

pub struct Options {
    args: HashMap<String, OptionType>
}

impl Options {

    fn new() -> Self {
        Self {
            args: HashMap::new()
        }
    }

    pub fn add_option(&mut self, flag: String, option: OptionType) -> Result<(), String> {

        if flag.starts_with("--") {
            self.args.insert(flag, option);
            Ok(())
        }
        else {
            Err(format!("Error: {} doesn't have a flag prefix", flag))
        }
    }

    pub fn has_option(&self, flag: &str) -> bool {
        self.args.contains_key(flag)
    }

    pub fn get_option_type(&self, flag: &str)  -> Result<OptionType, String> {
        self.args
            .get(flag)
            .cloned()
            .ok_or_else(|| format!("Unknown option: {}", flag))
    }
}