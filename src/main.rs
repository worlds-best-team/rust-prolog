use std::io::Write;
mod degc_degf;
mod error_msg;

fn main() {
    print!("Hello, World!\n\n");

    'crankshaft: loop {
        println!("Options:");
        println!(" 1. degc_degf::run_module()");
        println!(" 2. fibonacci::run_module()");
        print!("Enter choice or `q` to exit: ");
        std::io::stdout().flush().expect(error_msg::FAILED_FLUSH);

        let mut choice: String = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect(error_msg::FAILED_READ_LINE);
        let choice: &str = choice.trim();

        match choice {
            "1" => 'module: loop {
                if !degc_degf::run_module() {
                    break 'module;
                }
            },

            "q" => {
                println!("\n[EXITING...]");
                break 'crankshaft;
            }

            _ => print!("Invalid choice, try again!\n\n"),
        }
    }
}
