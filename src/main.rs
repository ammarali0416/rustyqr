use colored::*;
use std::io::Write;
use qrcode::QrCode;
use qrcode::render::unicode;
use crossterm::{terminal, ExecutableCommand};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::env;
use std::path::PathBuf;

fn print_colorful_title() {
    let ascii_art = [
        " ███████████                       █████                     ██████    ███████████  ",
        "▒▒███▒▒▒▒▒███                     ▒▒███                    ███▒▒▒▒███ ▒▒███▒▒▒▒▒███ ",
        " ▒███    ▒███  █████ ████  █████  ███████   █████ ████    ███    ▒▒███ ▒███    ▒███ ",
        " ▒██████████  ▒▒███ ▒███  ███▒▒  ▒▒▒███▒   ▒▒███ ▒███    ▒███     ▒███ ▒██████████  ",
        " ▒███▒▒▒▒▒███  ▒███ ▒███ ▒▒█████   ▒███     ▒███ ▒███    ▒███   ██▒███ ▒███▒▒▒▒▒███ ",
        " ▒███    ▒███  ▒███ ▒███  ▒▒▒▒███  ▒███ ███ ▒███ ▒███    ▒▒███ ▒▒████  ▒███    ▒███ ",
        " █████   █████ ▒▒████████ ██████   ▒▒█████  ▒▒███████     ▒▒▒██████▒██ █████   █████",
        "▒▒▒▒▒   ▒▒▒▒▒   ▒▒▒▒▒▒▒▒ ▒▒▒▒▒▒     ▒▒▒▒▒    ▒▒▒▒▒███       ▒▒▒▒▒▒ ▒▒ ▒▒▒▒▒   ▒▒▒▒▒ ",
        "                                             ███ ▒███                               ",
        "                                            ▒▒██████                                ",
        "                                             ▒▒▒▒▒▒                                 ",];

    for (i, line) in ascii_art.iter().enumerate() {
        let colored_line = match i % 6 {
            0 => line.red().bold(),
            1 => line.yellow().bold(),
            2 => line.green().bold(),
            3 => line.cyan().bold(),
            4 => line.blue().bold(),
            5 => line.magenta().bold(),
            _ => line.white().bold(),
        };
        println!("{}", colored_line);
    }
}

fn interactive_menu() -> u8 {
    let options = ["Output QR code in terminal", "Save QR code as PNG file"];
    let mut selected = 0;
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        println!("\nChoose an option (use ↑/↓ and Enter):");
        for (i, opt) in options.iter().enumerate() {
            if i == selected {
                println!("{} {}", "▶".green().bold(), opt.green().bold());
            } else {
                println!("  {}", opt);
            }
        }
        match read().unwrap() {
            Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                if selected > 0 { selected -= 1; }
            },
            Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                if selected < options.len() - 1 { selected += 1; }
            },
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                terminal::disable_raw_mode().unwrap();
                return (selected + 1) as u8;
            },
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: _, state: _ }) => {
                terminal::disable_raw_mode().unwrap();
                std::process::exit(0);
            },
            _ => {}
        }
    }
}

fn save_qr_png(code: &QrCode) {
    use image::Luma;
    use std::io::{self, Write};
    println!("Enter file path to save PNG (leave blank for default in Downloads): ");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read input");
    let path = path.trim();
    let file_path = if path.is_empty() {
        let mut downloads = dirs::download_dir().unwrap_or_else(|| env::current_dir().unwrap());
        downloads.push("rustyqr_output.png");
        downloads
    } else {
        PathBuf::from(path)
    };
    let image = code.render::<Luma<u8>>().min_dimensions(256, 256).build();
    match image.save(&file_path) {
        Ok(_) => println!("Saved QR code to: {}", file_path.display()),
        Err(e) => println!("Failed to save PNG: {}", e),
    }
}

fn main() {
    print_colorful_title();
    print!("\nEnter a URL to encode as a QR code: ");
    std::io::stdout().flush().unwrap();
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).expect("Failed to read input");
    let url = url.trim();
    // Clear any pending input events
    while crossterm::event::poll(std::time::Duration::from_millis(0)).unwrap() {
        let _ = crossterm::event::read();
    }
    // Generate QR code
    let code = QrCode::new(url).expect("Failed to generate QR code");
    let string = code.render::<unicode::Dense1x2>().build();
    // Interactive menu
    let choice = interactive_menu();
    match choice {
        1 => {
            println!("\n╔════════════════════════════════╗");
            println!("║         QR CODE BELOW          ║");
            println!("╚════════════════════════════════╝\n");
            println!("{}", string);
            println!("╔════════════════════════════════╗");
            println!("║      SCAN ME IF YOU DARE!      ║");
            println!("╚════════════════════════════════╝");
        },
        2 => {
            save_qr_png(&code);
        },
        _ => {
            let error_string: ColoredString = "Invalid choice. Please run the program again and select 1 or 2.".red();
            println!("{}", error_string);
        }
    }
    // Wait for any key to exit
    println!("\nPress any key to exit...");
    let _ = crossterm::event::read();
}
