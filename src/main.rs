use figlet_rs::FIGfont;
use std::env;

fn convert_to_figlet(text: &str) -> String {
    let standard_font: FIGfont = FIGfont::standard().unwrap();
    let mut lines: Vec<String> = vec!["".to_string(); standard_font.convert("A").unwrap().to_string().lines().count()];

    for char in text.chars() {
        if char.is_uppercase() {
            let rendered_char: String = standard_font.convert(&char.to_string()).unwrap().to_string();
            for (i, line) in rendered_char.lines().enumerate() {
                if i < lines.len() {
                    lines[i].push_str(line);
                }
            }
        } else {
            for i in 0..lines.len() {
                if i == lines.len() - 2 {
                    lines[i].push(char);
                } else {
                    lines[i].push(' ');
                }
            }
        }
    }

    lines.join("\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <text>", args[0]);
        std::process::exit(1);
    }

    let text: &String = &args[1];
    let converted_text: String = convert_to_figlet(text);
    println!("{}", converted_text);
}