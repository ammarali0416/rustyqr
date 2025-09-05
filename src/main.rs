use colored::*;
use crossterm::{
    ExecutableCommand,
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use qrcode::QrCode;
use qrcode::render::unicode;
use std::io::Write;

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
        "                                             ▒▒▒▒▒▒                                 ",
    ];

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
    enable_raw_mode().unwrap();
    loop {
        // Only redraw the menu (no full clear)
        stdout.execute(MoveTo(0, 0)).unwrap(); // ✅ THIS WORKS NOW
        stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();

        println!("Choose an option (use ↑/↓ and Enter):");
        for (i, opt) in options.iter().enumerate() {
            if i == selected {
                println!("{} {}", "▶".green().bold(), opt.green().bold());
            } else {
                println!("  {}", opt);
            }
        }

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if selected < options.len() - 1 {
                    selected += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                disable_raw_mode().unwrap();
                return (selected + 1) as u8;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                disable_raw_mode().unwrap();
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn save_qr_png(code: &QrCode) {
    use image::Luma;

    let mut downloads = dirs::download_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    downloads.push("rustyqr_output.png");
    let file_path = downloads;

    let image = code.render::<Luma<u8>>().min_dimensions(256, 256).build();

    match image.save(&file_path) {
        Ok(_) => println!("Saved QR code to: {}", file_path.display()),
        Err(e) => println!("Failed to save PNG: {}", e),
    }
}

fn read_line_crossterm() -> String {
    use crossterm::event::{Event, KeyCode, read};
    use crossterm::{cursor, execute, style::Print};
    use std::io::{Write, stdout};

    let mut input = String::new();
    let mut stdout = stdout();

    enable_raw_mode().unwrap();

    loop {
        if let Event::Key(key_event) = read().unwrap() {
            match key_event.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    execute!(stdout, Print(c)).unwrap();
                    stdout.flush().unwrap();
                }
                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        execute!(stdout, cursor::MoveLeft(1), Print(" "), cursor::MoveLeft(1))
                            .unwrap();
                    }
                }
                KeyCode::Enter => {
                    println!();
                    break;
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    input
}

fn main() {
    print_colorful_title();
    print!("\nEnter a URL to encode as a QR code: ");
    std::io::stdout().flush().unwrap();
    let mut url = String::new();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Failed to read input");
    let url = read_line_crossterm();
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
        }
        2 => {
            save_qr_png(&code);
        }
        _ => {
            let error_string: ColoredString =
                "Invalid choice. Please run the program again and select 1 or 2.".red();
            println!("{}", error_string);
        }
    }
    // Wait for any key to exit
    println!("\nPress any key to exit...");
    enable_raw_mode().unwrap();
    loop {
        if let Event::Key(key_event) = crossterm::event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                break;
            }
        }
    }
    disable_raw_mode().unwrap();

}
