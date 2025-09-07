# 📦 RustyQR

**A tiny Rust-powered QR code generator that lives on your terminal.**  
No accounts. No tracking. No “Pay to keep your QR alive” nonsense. Just free, local, and yours.

![RustyQR Terminal Screenshot](/docs/screenshot.png) <!-- Optional: add an actual screenshot -->

---

## 🚀 The Story (Why RustyQR Exists)

A few months ago, I used a site called [qrco.de](https://qrco.de) to generate a QR code linking to my LinkedIn profile.  
I slapped it on a business card template, ordered 50 cards from Canva, and felt like a pro.  

Two weeks later... someone scanned the card — and the QR **was dead**.  
The website had locked it behind a subscription.

💸 They wanted me to **pay** to keep my own QR code alive.  
🤦🏽 I printed business cards with **unusable** QR codes.

But we have QR code generation libraries right?

So I built **RustyQR** — a simple Rust terminal app that:
- Takes a URL (or any text)
- Lets you choose:
  - Print it in the terminal
  - Save it as a PNG
- Never breaks (hopefully).
- Never charges you a dime.

---

## 🧪 Try It Now (No Setup)

Just download the `.exe` file below and run it directly in your terminal:

📦 [Download RustyQR.exe](https://github.com/yourusername/rustyqr/releases/latest)  
> ✅ Works on Windows (no install needed)
> Maybe works on Mac/Linux?

---

## 💻 Features

- Generate QR codes *offline*
- Render directly in the terminal (ANSI-compatible)
- Save high-quality `.png` files
---

## 🔧 Building From Source

If you're a Rustacean:

```sh
git clone https://github.com/yourusername/rustyqr
cd rustyqr
cargo run
```

To create the .exe:
```sh
cargo build --release
```

Then copy target/release/rustyqr.exe into the root of your repo (if .gitignore skips it), or attach it to your GitHub Release manually.

## Contributing
Fork the repo, make your feature, then open a PR. Just make sure it doesn't break anything :)


## License

MIT. Free forever.
