use rand::Rng;

use crate::kbs2::config;
use crate::kbs2::error::Error;
use crate::kbs2::util;

pub trait Generator {
    fn name(&self) -> &str;
    fn secret(&self) -> Result<String, Error>;
}

impl Generator for config::GeneratorCommandConfig {
    fn name(&self) -> &str {
        &self.name
    }

    fn secret(&self) -> Result<String, Error> {
        let (command, args) = util::parse_and_split_args(&self.command)?;
        let args = args.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

        util::run_with_output(&command, &args)
    }
}

impl Generator for config::GeneratorInternalConfig {
    fn name(&self) -> &str {
        &self.name
    }

    fn secret(&self) -> Result<String, Error> {
        // NOTE(ww): Disallow non-ASCII, to prevent gibberish indexing below.
        if !self.alphabet.is_ascii() {
            return Err("generator alphabet contains non-ascii characters".into());
        }

        let mut rng = rand::thread_rng();
        let alphabet = self.alphabet.as_bytes();
        let secret = (0..self.length)
            .map(|_| alphabet[rng.gen_range(0, alphabet.len())] as char)
            .collect::<String>();

        Ok(secret)
    }
}