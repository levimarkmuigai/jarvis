mod cli;
mod domain;
mod api;
mod errors;

use domain::JarvisService;
use cli::Cli;
use errors::JarvisError;
use api::HttpAIClient;

fn main() -> Result<(), JarvisError> {

    dotenvy::dotenv().ok();

    let client = HttpAIClient::new()?;

    let jarvis = JarvisService::new(client);

    let prompt = match Cli::read_prompt() {
        Ok(p) => p,
        Err(e) => {
            eprint!("Error reading prompt: {}", e);
            return Err(e);
        }
    };

    let response = match jarvis.ask(&prompt) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Error talking to AI: {}", e);
            return Err(e);
        }
    };

    println!("{}", response.value());

    Ok(())
}
