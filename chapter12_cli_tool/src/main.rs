

use std::env;
use std::fs;

fn main() {
    /**
     * Bascic version:
     * IMPRROVING POINTS:
     *      1. ORAGNIZE args parsing feature
     *      2. ERROR HANDLING for file for args parsing fail + file reading fail
     */
    {
        let args: Vec<String> = env::args().collect();
        let query = &args[1];
        let filename = &args[2];
        print!("Query keyword '{}' from '{}'\n", &query, &filename);
        let content_result = fs::read_to_string(format!("./{}", &filename));
        let content = content_result.expect("Some issue happens\n");
        print!("Basic version: {}\n\n", content)
    }


    /**
     * Advanced version
     */
    {
        use std::process;
        struct Config {
            query: String,
            filename: String,
        }
        fn parse_args(args: Vec<String>) -> Result<Config, String> {
            if (args.len() <2) {
                return Err(String::from("Parse args error, please input at least two arguments\n"))
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            let config = Config {query, filename};
            Ok(config)
        }
        
        fn run() -> Result<(), String> {
            let args: Vec<String> = env::args().collect();
            let config = parse_args(args)?;
            let content = fs::read_to_string(format!("./{}", config.filename)).unwrap_or_else(|e|{
                print!("Error occurs (read_file): {}", e);
                process::exit(1);
            });
            print!("Classified version: {:?}\n\n", content);
            Ok(())
        }
        run().unwrap_or_else(|e|{
            print!("Error occurs (run): {}", e);
                process::exit(1);
        })
        
    }
}
