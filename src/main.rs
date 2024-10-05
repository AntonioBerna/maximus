use std::env;
use figlet_rs::FIGfont;

struct FigletConverter {
    font: FIGfont,
}

impl FigletConverter {
    fn new() -> Result<Self, &'static str> {
        match FIGfont::standard() {
            Ok(font) => Ok(FigletConverter { font }),
            Err(_) => Err("Failed to load standard FIGlet font"),
        }
    }

    fn convert(&self, text: &str) -> String {
        let mut lines: Vec<String> = vec![
            String::new();
            self.font.convert("A").unwrap().to_string().lines().count()
        ];

        for ch in text.chars() {
            if ch.is_uppercase() {
                if let Some(rendered_char) = self.font.convert(&ch.to_string()) {
                    for (i, line) in rendered_char.to_string().lines().enumerate() {
                        lines[i].push_str(line);
                    }
                }
            } else {
                for i in 0..lines.len() {
                    if i == lines.len() - 2 {
                        lines[i].push(ch);
                    } else {
                        lines[i].push(' ');
                    }
                }
            }
        }

        lines.join("\n")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <text>", args[0]);
        std::process::exit(1);
    }

    let converter: FigletConverter = FigletConverter::new().expect("Failed to create FIGlet converter");
    let converted_text: String = converter.convert(&args[1]);
    
    println!("{}", converted_text);
}