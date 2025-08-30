use crate::errors::JarvisError;
use crate::domain::Prompt;
use std::env::args;

pub struct Cli{}

impl Cli {
    pub fn read_prompt() -> Result<Prompt, JarvisError>{

        let args: Vec<String> = args().skip(1).collect();

        let full_prompt = args.join(" ");

        Prompt::new(full_prompt)
    }
}
