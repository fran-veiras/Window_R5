use std::fs::{self, File};
use std::fs::OpenOptions;
use std::io::Write;
mod routes;
use std::fs::read_to_string;
use std::error::Error;

fn main() {
    routes::print_config();

    fn prompt(route:&str) -> String {
        let mut route_app = String::new();
        print!("{}", route);
        std::io::stdout().flush().unwrap();
        
        std::io::stdin().read_line(&mut route_app).expect("Error: no se pudo leer la linea");
     
        return route_app.trim().to_string()
    }

    let mut input:String = prompt("> ");

    if input == "add" {
        write_document(input);
    } else if input == "end" {
        
    }

    fn write_document(mut input: String) -> std::io::Result<()> {
        let mut newRoute:String = prompt("> ");

        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .write(true)
            .create(true)
            .open("routes.txt");

        writeln!(file?, "{}", newRoute)?;

        let mut newPrompt:String = prompt("> ");

        if newPrompt == "end" {} else if newPrompt == "add" {write_document(newPrompt);};

        Ok(())
    }

    fn read_document() -> Result<File, std::io::Error>: AsRef<Path> {
        let mut file = OpenOptions::new().read(true).open("routes.txt");

        let contents = fs::read_to_string(file);

    }

    //println!("Abriendo... {}", input);

    // abrir app -> let result = opener::open(std::path::Path::new(&input));

    // println!("{:?}", result); 
}
