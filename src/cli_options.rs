use std::collections::HashMap;
use std::any::Any;

/*
    - Короткие флаги, нужна в принципе возможность получать инфу есть ли флаг в аргументах
*/

pub struct Options {
    m_args: HashMap<String, String>
}

/*pub fn new() -> Result<Options, String> {
}
*/
impl Options {
    pub fn add_option(mut self, flag: String, option: Option<String>) {
        self.m_args.insert(flag, option.unwrap_or("".to_string()));
    }

    
}