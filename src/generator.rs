use std::borrow::Cow;
use std::fmt::Write as FmtWrite;
use eframe::egui::Color32;
use image::{Rgba, RgbaImage};
use qrcode::{QrCode, Color as QrColor};
use crate::types::{
    ContentKind, CryptoCurrency, CryptoData, EcLevelSetting, EmailData, EyeShape,
    GeoData, ModuleShape, SmsData, StyleConfig, VCardData, WifiData, WifiEncryption,
};

#[derive(Clone, Debug)]
pub struct QrMatrix {
    pub width: usize,
    pub data: Vec<bool>, // row-major, true = dark
    pub version: qrcode::Version,
}

impl QrMatrix {
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.width {
            self.data[y * self.width + x]
        } else {
            false
        }
    }

    pub fn is_in_finder_pattern(&self, x: usize, y: usize) -> bool {
        let w = self.width;
        let in_top_left = x < 7 && y < 7;
        let in_top_right = x >= w - 7 && y < 7;
        let in_bottom_left = x < 7 && y >= w - 7;
        in_top_left || in_top_right || in_bottom_left
    }

    #[allow(dead_code)]
    pub fn finder_center(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        let w = self.width as f32;
        let xf = x as f32;
        let yf = y as f32;
        if xf < 7.0 && yf < 7.0 {
            Some((3.5, 3.5))
        } else if xf >= w - 7.0 && yf < 7.0 {
            Some((w - 3.5, 3.5))
        } else if xf < 7.0 && yf >= w - 7.0 {
            Some((3.5, w - 3.5))
        } else {
            None
        }
    }
}

pub fn build_payload(
    kind: ContentKind,
    url: &str,
    text: &str,
    wifi: &WifiData,
    vcard: &VCardData,
    email: &EmailData,
    phone: &str,
    sms: &SmsData,
    geo: &GeoData,
    crypto: &CryptoData,
) -> String {
    match kind {
        ContentKind::Url => {
            let trimmed = url.trim();
            if trimmed.is_empty() {
                "https://example.com".to_string()
            } else if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
                format!("https://{}", trimmed)
            } else {
                trimmed.to_string()
            }
        }
        ContentKind::Text => {
            if text.trim().is_empty() {
                "Hello World".to_string()
            } else {
                text.to_string()
            }
        }
        ContentKind::Wifi => {
            let enc = match wifi.encryption {
                WifiEncryption::Wpa => "WPA",
                WifiEncryption::Wep => "WEP",
                WifiEncryption::None => "nopass",
            };
            let escape = |s: &str| -> String {
                s.replace('\\', "\\\\")
                    .replace(';', "\\;")
                    .replace(',', "\\,")
                    .replace(':', "\\:")
                    .replace('"', "\\\"")
            };
            format!(
                "WIFI:T:{};S:{};P:{};H:{};;",
                enc,
                escape(&wifi.ssid),
                escape(&wifi.password),
                if wifi.hidden { "true" } else { "false" }
            )
        }
        ContentKind::VCard => {
            format!(
                "BEGIN:VCARD\r\nVERSION:3.0\r\nN:{};{};;;\r\nFN:{} {}\r\nORG:{}\r\nTITLE:{}\r\nTEL;TYPE=CELL:{}\r\nEMAIL:{}\r\nURL:{}\r\nNOTE:{}\r\nEND:VCARD",
                vcard.last_name.trim(),
                vcard.first_name.trim(),
                vcard.first_name.trim(),
                vcard.last_name.trim(),
                vcard.organization.trim(),
                vcard.job_title.trim(),
                vcard.phone.trim(),
                vcard.email.trim(),
                vcard.website.trim(),
                vcard.note.trim()
            )
        }
        ContentKind::Email => {
            let mut res = format!("mailto:{}", email.to.trim());
            let mut params = Vec::new();
            if !email.subject.trim().is_empty() {
                params.push(format!("subject={}", urlencoding(email.subject.trim())));
            }
            if !email.body.trim().is_empty() {
                params.push(format!("body={}", urlencoding(email.body.trim())));
            }
            if !params.is_empty() {
                res.push('?');
                res.push_str(&params.join("&"));
            }
            res
        }
        ContentKind::Phone => {
            let clean: String = phone.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
            format!("tel:{}", clean)
        }
        ContentKind::Sms => {
            let clean: String = sms.phone.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect();
            if sms.message.trim().is_empty() {
                format!("sms:{}", clean)
            } else {
                format!("sms:{}?body={}", clean, urlencoding(sms.message.trim()))
            }
        }
        ContentKind::Geo => {
            format!("geo:{:.6},{:.6}", geo.latitude, geo.longitude)
        }
        ContentKind::Crypto => {
            let prefix = match crypto.currency {
                CryptoCurrency::Bitcoin => "bitcoin",
                CryptoCurrency::Ethereum => "ethereum",
                CryptoCurrency::Solana => "solana",
                CryptoCurrency::Usdt => "tether",
            };
            let addr = crypto.address.trim();
            if crypto.amount.trim().is_empty() {
                format!("{}:{}", prefix, addr)
            } else {
                match crypto.currency {
                    CryptoCurrency::Bitcoin | CryptoCurrency::Solana => {
                        format!("{}:{}?amount={}", prefix, addr, crypto.amount.trim())
                    }
                    CryptoCurrency::Ethereum => {
                        format!("{}:{}?value={}", prefix, addr, crypto.amount.trim())
                    }
                    CryptoCurrency::Usdt => {
                        format!("{}:{}?amount={}", prefix, addr, crypto.amount.trim())
                    }
                }
            }
        }
    }
}

fn urlencoding(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' || byte == b'~' {
            encoded.push(byte as char);
        } else {
            let _ = write!(encoded, "%{:02X}", byte);
        }
    }
    encoded
}

pub fn generate_matrix(content: &str, ec: EcLevelSetting) -> Result<QrMatrix, String> {
    if content.is_empty() {
        return Err("Payload cannot be empty".to_string());
    }
    let code = QrCode::with_error_correction_level(content.as_bytes(), ec.to_qrcode_level())
        .map_err(|e| format!("Failed to generate QR code: {}", e))?;

    let width = code.width();
    let colors = code.to_colors();
    let data: Vec<bool> = colors
        .into_iter()
        .map(|c| c == QrColor::Dark)
        .collect();

    Ok(QrMatrix {
        width,
        data,
        version: code.version(),
    })
}

fn color_to_hex(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

pub fn render_svg(matrix: &QrMatrix, style: &StyleConfig) -> String {
    let margin = style.margin as f32;
    let w = matrix.width as f32;
    let total_size = w + margin * 2.0;

    let fg_hex = color_to_hex(style.fg_color);
    let bg_hex = color_to_hex(style.bg_color);

    let mut svg = String::with_capacity(8192);
    let _ = write!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {total} {total}" width="{res}" height="{res}">"#,
        total = total_size,
        res = style.export_size
    );

    if !style.transparent_bg {
        let _ = write!(
            svg,
            r#"<rect width="100%" height="100%" fill="{}" />"#,
            bg_hex
        );
    }

    let _ = write!(svg, r#"<g fill="{}">"#, fg_hex);

    for y in 0..matrix.width {
        for x in 0..matrix.width {
            if !matrix.is_dark(x, y) {
                continue;
            }

            let mx = x as f32 + margin;
            let my = y as f32 + margin;
            let is_finder = matrix.is_in_finder_pattern(x, y);

            let shape = if is_finder {
                match style.eye_shape {
                    EyeShape::Unified => style.module_shape,
                    EyeShape::RoundedEye => ModuleShape::Rounded,
                    EyeShape::CircleEye => ModuleShape::Circle,
                }
            } else {
                style.module_shape
            };

            match shape {
                ModuleShape::Square => {
                    let _ = write!(
                        svg,
                        r#"<rect x="{:.2}" y="{:.2}" width="1.02" height="1.02" />"#,
                        mx, my
                    );
                }
                ModuleShape::Rounded => {
                    let _ = write!(
                        svg,
                        r#"<rect x="{:.2}" y="{:.2}" width="0.9" height="0.9" rx="0.25" ry="0.25" />"#,
                        mx + 0.05,
                        my + 0.05
                    );
                }
                ModuleShape::Circle => {
                    let _ = write!(
                        svg,
                        r#"<circle cx="{:.2}" cy="{:.2}" r="0.45" />"#,
                        mx + 0.5,
                        my + 0.5
                    );
                }
                ModuleShape::SoftSquare => {
                    let _ = write!(
                        svg,
                        r#"<rect x="{:.2}" y="{:.2}" width="0.92" height="0.92" rx="0.38" ry="0.38" />"#,
                        mx + 0.04,
                        my + 0.04
                    );
                }
            }
        }
    }

    let _ = write!(svg, "</g></svg>");
    svg
}

pub fn render_image(matrix: &QrMatrix, style: &StyleConfig, target_size: u32) -> RgbaImage {
    let margin = style.margin as f32;
    let grid_dim = matrix.width as f32 + margin * 2.0;

    let cell_px = (target_size as f32 / grid_dim).max(1.0);
    let actual_size = (grid_dim * cell_px).round() as u32;

    let mut img = RgbaImage::new(actual_size, actual_size);

    let bg = if style.transparent_bg {
        Rgba([0, 0, 0, 0])
    } else {
        Rgba([
            style.bg_color.r(),
            style.bg_color.g(),
            style.bg_color.b(),
            style.bg_color.a(),
        ])
    };

    let fg = Rgba([
        style.fg_color.r(),
        style.fg_color.g(),
        style.fg_color.b(),
        style.fg_color.a(),
    ]);

    // Fill background
    for pixel in img.pixels_mut() {
        *pixel = bg;
    }

    let ss = 2; // 2x supersampling for high fidelity smooth shapes
    let sub_step = 1.0 / (ss as f32);

    for my in 0..matrix.width {
        for mx in 0..matrix.width {
            if !matrix.is_dark(mx, my) {
                continue;
            }

            let is_finder = matrix.is_in_finder_pattern(mx, my);
            let shape = if is_finder {
                match style.eye_shape {
                    EyeShape::Unified => style.module_shape,
                    EyeShape::RoundedEye => ModuleShape::Rounded,
                    EyeShape::CircleEye => ModuleShape::Circle,
                }
            } else {
                style.module_shape
            };

            let start_x = ((mx as f32 + margin) * cell_px).floor() as u32;
            let start_y = ((my as f32 + margin) * cell_px).floor() as u32;
            let end_x = (((mx as f32 + margin + 1.0) * cell_px).ceil() as u32).min(actual_size);
            let end_y = (((my as f32 + margin + 1.0) * cell_px).ceil() as u32).min(actual_size);

            match shape {
                ModuleShape::Square => {
                    for py in start_y..end_y {
                        for px in start_x..end_x {
                            img.put_pixel(px, py, fg);
                        }
                    }
                }
                ModuleShape::Circle | ModuleShape::Rounded | ModuleShape::SoftSquare => {
                    let cell_origin_x = (mx as f32 + margin) * cell_px;
                    let cell_origin_y = (my as f32 + margin) * cell_px;
                    let radius_mult = match shape {
                        ModuleShape::Circle => 0.46,
                        ModuleShape::Rounded => 0.45,
                        _ => 0.46,
                    };
                    let corner_radius = match shape {
                        ModuleShape::Rounded => 0.25 * cell_px,
                        ModuleShape::SoftSquare => 0.38 * cell_px,
                        _ => 0.0,
                    };

                    for py in start_y..end_y {
                        for px in start_x..end_x {
                            let mut covered_samples = 0;
                            for sy in 0..ss {
                                for sx in 0..ss {
                                    let sample_x = (px as f32 + (sx as f32 + 0.5) * sub_step) - cell_origin_x;
                                    let sample_y = (py as f32 + (sy as f32 + 0.5) * sub_step) - cell_origin_y;

                                    let inside = match shape {
                                        ModuleShape::Circle => {
                                            let cx = cell_px * 0.5;
                                            let cy = cell_px * 0.5;
                                            let dx = sample_x - cx;
                                            let dy = sample_y - cy;
                                            (dx * dx + dy * dy) <= (cell_px * radius_mult).powi(2)
                                        }
                                        ModuleShape::Rounded | ModuleShape::SoftSquare => {
                                            let pad = cell_px * 0.05;
                                            let box_w = cell_px - pad * 2.0;
                                            let lx = sample_x - pad;
                                            let ly = sample_y - pad;
                                            if lx >= 0.0 && lx <= box_w && ly >= 0.0 && ly <= box_w {
                                                let cr = corner_radius;
                                                let in_corner = (lx < cr && ly < cr)
                                                    || (lx > box_w - cr && ly < cr)
                                                    || (lx < cr && ly > box_w - cr)
                                                    || (lx > box_w - cr && ly > box_w - cr);

                                                if in_corner {
                                                    let cx = if lx < cr { cr } else { box_w - cr };
                                                    let cy = if ly < cr { cr } else { box_w - cr };
                                                    let dx = lx - cx;
                                                    let dy = ly - cy;
                                                    (dx * dx + dy * dy) <= cr * cr
                                                } else {
                                                    true
                                                }
                                            } else {
                                                false
                                            }
                                        }
                                        _ => true,
                                    };

                                    if inside {
                                        covered_samples += 1;
                                    }
                                }
                            }

                            if covered_samples > 0 {
                                let total_samples = (ss * ss) as f32;
                                let coverage = (covered_samples as f32) / total_samples;
                                if coverage >= 0.999 {
                                    img.put_pixel(px, py, fg);
                                } else {
                                    // Blend with background
                                    let orig = *img.get_pixel(px, py);
                                    let blended = blend_pixel(orig, fg, coverage);
                                    img.put_pixel(px, py, blended);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    img
}

fn blend_pixel(bg: Rgba<u8>, fg: Rgba<u8>, factor: f32) -> Rgba<u8> {
    let inv = 1.0 - factor;
    let r = (fg[0] as f32 * factor + bg[0] as f32 * inv).round() as u8;
    let g = (fg[1] as f32 * factor + bg[1] as f32 * inv).round() as u8;
    let b = (fg[2] as f32 * factor + bg[2] as f32 * inv).round() as u8;
    let a = (fg[3] as f32 * factor + bg[3] as f32 * inv).round() as u8;
    Rgba([r, g, b, a])
}

pub fn copy_image_to_clipboard(image: &RgbaImage) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("Clipboard error: {}", e))?;
    
    let (w, h) = image.dimensions();
    let img_data = arboard::ImageData {
        width: w as usize,
        height: h as usize,
        bytes: Cow::Borrowed(image.as_raw()),
    };

    clipboard.set_image(img_data)
        .map_err(|e| format!("Failed to set clipboard image: {}", e))?;

    Ok(())
}

pub fn copy_text_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("Clipboard error: {}", e))?;
    clipboard.set_text(text.to_string())
        .map_err(|e| format!("Failed to set clipboard text: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_payload() {
        let p1 = build_payload(
            ContentKind::Url,
            "example.com",
            "",
            &WifiData::default(),
            &VCardData::default(),
            &EmailData::default(),
            "",
            &SmsData::default(),
            &GeoData::default(),
            &CryptoData::default(),
        );
        assert_eq!(p1, "https://example.com");
    }

    #[test]
    fn test_wifi_payload() {
        let mut wifi = WifiData::default();
        wifi.ssid = "Home;Wifi".to_string();
        wifi.password = "pass:123".to_string();
        let p = build_payload(
            ContentKind::Wifi,
            "",
            "",
            &wifi,
            &VCardData::default(),
            &EmailData::default(),
            "",
            &SmsData::default(),
            &GeoData::default(),
            &CryptoData::default(),
        );
        assert_eq!(p, "WIFI:T:WPA;S:Home\\;Wifi;P:pass\\:123;H:false;;");
    }

    #[test]
    fn test_vcard_payload() {
        let vcard = VCardData::default();
        let p = build_payload(
            ContentKind::VCard,
            "",
            "",
            &WifiData::default(),
            &vcard,
            &EmailData::default(),
            "",
            &SmsData::default(),
            &GeoData::default(),
            &CryptoData::default(),
        );
        assert!(p.starts_with("BEGIN:VCARD\r\nVERSION:3.0"));
        assert!(p.contains("FN:John Doe"));
        assert!(p.ends_with("END:VCARD"));
    }

    #[test]
    fn test_all_shapes_and_palettes() {
        let mat = generate_matrix("https://github.com", EcLevelSetting::High).expect("generate QR");
        for shape in [
            ModuleShape::Square,
            ModuleShape::Rounded,
            ModuleShape::Circle,
            ModuleShape::SoftSquare,
        ] {
            let mut style = StyleConfig::default();
            style.module_shape = shape;
            style.transparent_bg = true;

            let svg = render_svg(&mat, &style);
            assert!(!svg.is_empty());

            let img = render_image(&mat, &style, 128);
            assert_eq!(img.width(), 128);
            assert_eq!(img.height(), 128);
        }
    }
}
