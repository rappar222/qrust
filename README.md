# QRust ⬛ — Studio QR Code Generator

A fast, modern desktop QR code generator written in **Rust** using **egui / eframe**. Features real-time live preview, custom styling palettes, dot shapes, transparent backgrounds, vCard contact cards, Wi-Fi presets, and high-resolution PNG & vector SVG export.

---

## ✨ Features

### 📋 Content Presets & Data Types
- 🌐 **Website / URL**: Generates standard web links with optional `https://` auto-prefixing.
- 📝 **Plain Text / Notes**: Freeform multiline text with live character counter.
- 📶 **Wi-Fi Network**: Fast connect QR codes (SSID, Password, WPA/WPA2/WPA3 / WEP / Open, and Hidden Network support).
- 📇 **vCard 3.0 Contact Card**: First/last name, phone, email, company, job title, website, and personal notes.
- 📧 **Email**: Pre-filled recipient, subject, and body (`mailto:` scheme).
- 📞 **Phone Number**: Direct telephone dialing link (`tel:` scheme).
- 💬 **SMS Message**: Mobile messaging format with recipient and pre-filled message text.
- 📍 **Geographic Location**: Latitude / longitude coordinates with one-click landmark buttons (Paris, New York, Tokyo, Rome).
- 🪙 **Cryptocurrency**: Addresses for Bitcoin (`BTC`), Ethereum (`ETH`), Solana (`SOL`), and Tether (`USDT`) with optional transfer amounts.

### 🎨 Visual Customization & Styling
- **Preset Color Palettes**:
  - *Classic* (Slate on White)
  - *Ocean Navy* (Royal Blue on Ice)
  - *Forest* (Emerald on Mint)
  - *Sunset* (Rose on Blush)
  - *Cyber Violet* (Purple on Lilac)
  - *Amber Warm* (Gold on Cream)
  - *Dark Neon* (Cyan on Slate)
- **Custom Color Pickers**: Individual foreground and background color pickers with hex support.
- **Transparent Background**: Toggle transparent background (rendered with checkerboard pattern preview).
- **Module Shapes**:
  - ⬛ *Sharp Square* (Standard crisp QR modules)
  - 🔲 *Rounded* (Modern rounded rectangles)
  - ⚪ *Circle / Dot* (Dot-matrix circular style)
  - 🔘 *Smooth Squircle* (Super-elliptical organic shape)
- **Corner Eyes (Finder Patterns)**:
  - *Match Modules* (Unified shape across the entire code)
  - *Rounded Outer* (Curved finder frames)
  - *Circular Target* (Round bullseye finder patterns)
- **Quiet Zone / Margin**: Slider from 0 to 8 modules of padding.
- **Error Correction Levels**:
  - `L` (~7% recovery) — Compact code
  - `M` (~15% recovery) — Recommended default
  - `Q` (~25% recovery) — Heavy-duty
  - `H` (~30% recovery) — Maximum durability

### 🚀 Export & Usability
- 📋 **Copy Image**: 1-click copy directly into your OS clipboard to paste into Slack, Discord, docs, or emails.
- 📄 **Copy Raw Payload**: Copies the encoded data string to the clipboard.
- 💾 **Save as PNG**: High-resolution bitmap export with configurable resolutions (512px, 1024px, 2048px, 4096px).
- 📐 **Save as SVG**: Pure vector format with exact geometries for print shops and graphic design.
- ☀️ / 🌙 **Dark & Light Mode**: Instant toggle for comfortable viewing.
- 🔄 **Quick Reset**: Reset all settings back to default anytime.

---

## 🛠️ Building & Running

### Requirements
- Rust 1.80+ (tested on Rust 1.99)
- Standard graphics dependencies (present on modern Linux, Windows, and macOS)

### Run in Debug Mode
```bash
cargo run
```

### Build Optimized Release Binary
```bash
cargo build --release
```

The resulting binary will be located at:
```bash
./target/release/qrust
```

---

## 📁 Architecture Overview

```
qrust/
├── Cargo.toml               # Dependencies (eframe, qrcode, image, rfd, arboard)
├── src/
│   ├── main.rs              # Application entry point, window options & icon
│   ├── app.rs               # Main GUI loop, layout, state & action handlers
│   ├── types.rs             # Content models, presets, palettes & style configurations
│   ├── generator.rs         # QrCode matrix generation, SVG & smooth antialiased rasterizer
│   └── ui/
│       ├── mod.rs           # UI module definitions
│       ├── content_form.rs  # Input controls for each QR data type
│       ├── style_panel.rs   # Color palettes, module shapes, margin, error correction
│       └── preview_panel.rs # Real-time preview canvas, stats, action buttons
```
