use std::io::Write;

fn main() {
    fn prompt(route:&str) -> String {
        let mut route_app = String::new();
        print!("{}", route);
        std::io::stdout().flush().unwrap();
        
        std::io::stdin().read_line(&mut route_app).expect("Error: no se pudo leer la linea");
     
        return route_app.trim().to_string()
    }

    let input = prompt("> ");

    println!("Abriendo... {}", input);

    let result = opener::open(std::path::Path::new(&input));

    println!("{:?}", result); 
}
