use std::fs::OpenOptions;
use std::io::Write;
mod routes;
mod utils;

fn main() {
    routes::print_config();

    fn prompt(route:&str) -> String {
        let mut route_app = String::new();
        print!("{}", route);
        std::io::stdout().flush().unwrap();
        
        std::io::stdin().read_line(&mut route_app).expect("Error: no se pudo leer la linea");
     
        return route_app.trim().to_string()
    }

    let input:String = prompt("> ");

    if input == "add" {
        write_document(input);
    } else if input == "run" {
        utils::open::open_files()
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

        writeln!(file?, "{};", newRoute)?;

        let mut newPrompt:String = prompt("> ");

        if newPrompt == "end" {} else if newPrompt == "add" {write_document(newPrompt);};

        Ok(())
    }

    //println!("Abriendo... {}", input);

    // abrir app -> let result = opener::open(std::path::Path::new(&input));

    // println!("{:?}", result); 
}
