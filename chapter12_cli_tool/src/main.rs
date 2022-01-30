



fn main() {
    /**
     * Bascic version:
     * IMPRROVING POINTS:
     *      1. ORAGNIZE args parsing feature
     *      2. ERROR HANDLING for file for args parsing fail + file reading fail
     */
    {
        use std::env;
        use std::fs;

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
     * 1. Create a Config struct for args, and impl a construct fucntion following the convention named as "new" to parse the arguments for a new Config struct
     * 2. Handle the Error when the length of args < 2
     * 3. Handle the Error when there is no such input file.
     */
    {
        use std::process;
        use std::env;
        use std::fs;

        struct Config {
            query: String,
            filename: String,
        }

        impl Config {
            fn new(args: Vec<String>) -> Result<Config, String> {
                if (args.len() <2) {
                    return Err(String::from("Parse args error, please input at least two arguments\n"))
                }
                let query = args[1].clone();
                let filename = args[2].clone();
                let config = Config {query, filename};
                Ok(config)
            }
        }        
        
        fn run() -> Result<(), String> {
            let args: Vec<String> = env::args().collect();
            let config = Config::new(args)?;
            let content = fs::read_to_string(format!("./{}", config.filename)).unwrap_or_else(|e|{
                print!("Error occurs (read_file): {}", e);
                process::exit(1);
            });
            print!("Classified version: {:?}\n\n", content);
            Ok(())
        }
        // run().unwrap_or_else(|e|{
        //     print!("Error occurs (run): {}", e);
        //         process::exit(1);
        // })
        if let Err(e) = run() {
            println!("Error occurs (run): {}", e);
            process::exit(1)
        }        
    }

    /**
     * Final version with test
     *      1. Move callable functions into library crate
     *      2. Create search function to search query keyword in passed in content
     *      3. Uint test the search functions 
     *      4. Use env::var to get enviornment variable to determinate if search result should be case sensitive
     */
    {
        use std::process;
        use chapter12_cli_tool::*;
        run().unwrap_or_else(|e|{
            print!("Error occurs (run): {}", e);
                process::exit(1);
        });
    }
}
