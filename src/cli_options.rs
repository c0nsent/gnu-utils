use std::collections::HashMap;

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