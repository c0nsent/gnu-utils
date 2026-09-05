use std::collections::HashMap;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgType {

}
pub struct Options {

    options:

/*    types: Vec<ArgType>,
    long_options: HashMap<String, usize>,
    short_options: HashMap<String, usize>,*/
}

impl Options {

}





/*pub struct Options {
    args: HashMap<String, ArgType>
}*/



// impl Options {
//
//     pub fn new() -> Self {
//         Self {
//             args: HashMap::new()
//         }
//     }
//
//     pub fn add_option(&mut self, flag: &str, option: ArgType) -> Result<(), String> {
//
//         if flag.starts_with("--") {
//             self.args.insert(flag, option);
//             Ok(())
//         }
//         else {
//             Err(format!("Error: {} doesn't have a flag prefix", flag))
//         }
//     }
//
//     pub fn has_option(&self, flag: &str) -> bool {
//         self.args.contains_key(flag)
//     }
//
//     pub fn get_option_type(&self, flag: &str)  -> Result<ArgType, String> {
//         self.args
//             .get(flag)
//             .cloned()
//             .ok_or_else(|| format!("Unknown option: {}", flag))
//     }
// }