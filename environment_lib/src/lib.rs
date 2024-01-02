use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub home_directory: String,
}


impl Environment {
    pub fn new() -> Self {
        let home = env::var("CHAIN_HOME").unwrap_or_else(|_| {
            // fall back to using cargo variables
            let path = env::var("OUT_DIR").unwrap_or_else(|_| {
                // fall back to using current path
                std::env::current_dir().unwrap().to_string_lossy().into_owned()
            });
            // assume either current path or cargo variables specify a sub path which needs to be modified
            format!("{}/../",path)
        });
        Environment { home_directory: home }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_config() {
        let environment = Environment::new();
        println!("Current directory: {}", &environment.home_directory);
    }
}
