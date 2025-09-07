use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use qrcode::QrCode;
use qrcode::render::unicode;
use std::io::{self, Write};
use std::time::Duration;

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
        "I hate you qrco.de - Ammar",
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
    let mut input = String::new();

    enable_raw_mode().unwrap();
    // Ensure we always restore the terminal, even on panic/return.
    struct RawGuard;
    impl Drop for RawGuard {
        fn drop(&mut self) {
            let _ = disable_raw_mode();
        }
    }
    let _guard = RawGuard;

    loop {
        // blocks until an event is available
        let ev = event::read().unwrap();
        match ev {
            Event::Key(key) => {
                // 🔑 IMPORTANT: only handle real key presses
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Enter => {
                        println!();
                        drain_pending_events();
                        break;
                    }
                    KeyCode::Backspace => {
                        if input.pop().is_some() {
                            // erase one char visually
                            print!("\x08 \x08");
                            io::stdout().flush().unwrap();
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        // manually echo because raw mode disables terminal echo
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }
                    _ => {}
                }
            }
            // Optional: handle paste events as a whole chunk (no double echo)
            Event::Paste(s) => {
                input.push_str(&s);
                print!("{}", s);
                io::stdout().flush().unwrap();
            }
            _ => {}
        }
    }

    input
}

fn drain_pending_events() {
    // Non-blocking: eat everything currently in the queue (including extra Enter / repeats)
    while event::poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = event::read();
    }
}

fn main() {
    print_colorful_title();
    print!("\nEnter a URL to encode as a QR code: ");
    std::io::stdout().flush().unwrap();
    // ✅ Read the URL **once** using your crossterm helper
    let mut url = read_line_crossterm();
    url = url.trim().to_string();

    // ✅ basic validation
    if url.is_empty() {
        eprintln!("No URL entered — aborting.");
        return;
    }

    // ✅ make sure phones treat it as a link
    // if !url.starts_with("http://") && !url.starts_with("https://") {
    //     url = format!("https://{}", url);
    // }

    // ✅ generate the QR from the (possibly normalized) URL
    let code = QrCode::new(&url).expect("Failed to generate QR code");
    let string = code.render::<unicode::Dense1x2>().build();
    // (interactive_menu removed: program is now non-interactive)
    println!("\n╔════════════════════════════════╗");
    println!("║         QR CODE BELOW          ║");
    println!("╚════════════════════════════════╝\n");
    println!("{}", string);
    println!("╔════════════════════════════════╗");
    println!("║      SCAN ME IF YOU DARE!      ║");
    println!("╚════════════════════════════════╝");

    // Save PNG automatically and show path
    save_qr_png(&code);

    // Press any key to exit
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
