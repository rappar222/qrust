use eframe::egui::{self, Ui};
use crate::types::{
    ContentKind, CryptoCurrency, CryptoData, EmailData, GeoData, SmsData, VCardData,
    WifiData, WifiEncryption,
};

pub fn render_content_form(
    ui: &mut Ui,
    kind: ContentKind,
    url: &mut String,
    text: &mut String,
    wifi: &mut WifiData,
    vcard: &mut VCardData,
    email: &mut EmailData,
    phone: &mut String,
    sms: &mut SmsData,
    geo: &mut GeoData,
    crypto: &mut CryptoData,
) {
    match kind {
        ContentKind::Url => {
            ui.label(egui::RichText::new("Website / Link URL").strong());
            ui.add(
                egui::TextEdit::singleline(url)
                    .hint_text("https://your-domain.com")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("🌐 Prepend https://").clicked() {
                    if !url.starts_with("http://") && !url.starts_with("https://") {
                        *url = format!("https://{}", url.trim());
                    }
                }
                if ui.button("Clear").clicked() {
                    url.clear();
                }
            });
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Scanning opens this URL directly in the mobile browser.")
                    .weak()
                    .small(),
            );
        }
        ContentKind::Text => {
            ui.label(egui::RichText::new("Plain Text / Note").strong());
            ui.add(
                egui::TextEdit::multiline(text)
                    .hint_text("Enter plain text, notes, coupon codes, serial numbers...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(6),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("Characters: {}", text.chars().count()))
                        .weak()
                        .small(),
                );
                if ui.button("Clear").clicked() {
                    text.clear();
                }
            });
        }
        ContentKind::Wifi => {
            ui.label(egui::RichText::new("Network Name (SSID)").strong());
            ui.add(
                egui::TextEdit::singleline(&mut wifi.ssid)
                    .hint_text("e.g. Office_Guest_WiFi")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Password").strong());
            ui.add(
                egui::TextEdit::singleline(&mut wifi.password)
                    .password(false)
                    .hint_text("Wi-Fi network password")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Security / Encryption").strong());
            ui.horizontal(|ui| {
                ui.selectable_value(&mut wifi.encryption, WifiEncryption::Wpa, "WPA/WPA2/WPA3");
                ui.selectable_value(&mut wifi.encryption, WifiEncryption::Wep, "WEP");
                ui.selectable_value(&mut wifi.encryption, WifiEncryption::None, "Open (No Pass)");
            });
            ui.add_space(6.0);

            ui.checkbox(&mut wifi.hidden, "Hidden Network (SSID not broadcasting)");
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Guests can scan to instantly connect without typing passwords.")
                    .weak()
                    .small(),
            );
        }
        ContentKind::VCard => {
            ui.label(egui::RichText::new("Contact Card (vCard)").strong());
            egui::Grid::new("vcard_grid")
                .num_columns(2)
                .spacing([8.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("First Name:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.first_name).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Last Name:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.last_name).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Company / Org:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.organization).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Job Title:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.job_title).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Phone:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.phone).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Email:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.email).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Website:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.website).desired_width(f32::INFINITY));
                    ui.end_row();

                    ui.label("Note:");
                    ui.add(egui::TextEdit::singleline(&mut vcard.note).desired_width(f32::INFINITY));
                    ui.end_row();
                });
        }
        ContentKind::Email => {
            ui.label(egui::RichText::new("Recipient Email").strong());
            ui.add(
                egui::TextEdit::singleline(&mut email.to)
                    .hint_text("support@example.com")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Subject").strong());
            ui.add(
                egui::TextEdit::singleline(&mut email.subject)
                    .hint_text("Subject line")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Body").strong());
            ui.add(
                egui::TextEdit::multiline(&mut email.body)
                    .hint_text("Pre-filled message content...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(4),
            );
        }
        ContentKind::Phone => {
            ui.label(egui::RichText::new("Phone Number").strong());
            ui.add(
                egui::TextEdit::singleline(phone)
                    .hint_text("+1 (555) 012-3456")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Scanning prompts mobile devices to immediately dial this number.")
                    .weak()
                    .small(),
            );
        }
        ContentKind::Sms => {
            ui.label(egui::RichText::new("Recipient Phone Number").strong());
            ui.add(
                egui::TextEdit::singleline(&mut sms.phone)
                    .hint_text("+1 (555) 012-3456")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("SMS Message").strong());
            ui.add(
                egui::TextEdit::multiline(&mut sms.message)
                    .hint_text("Text message to send...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(3),
            );
        }
        ContentKind::Geo => {
            ui.label(egui::RichText::new("Geographic Coordinates").strong());
            egui::Grid::new("geo_grid")
                .num_columns(2)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Latitude:");
                    ui.add(
                        egui::DragValue::new(&mut geo.latitude)
                            .speed(0.0001)
                            .range(-90.0..=90.0)
                            .custom_formatter(|n, _| format!("{:.6}°", n)),
                    );
                    ui.end_row();

                    ui.label("Longitude:");
                    ui.add(
                        egui::DragValue::new(&mut geo.longitude)
                            .speed(0.0001)
                            .range(-180.0..=180.0)
                            .custom_formatter(|n, _| format!("{:.6}°", n)),
                    );
                    ui.end_row();
                });
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Quick Landmarks:").small().strong());
            ui.horizontal_wrapped(|ui| {
                if ui.button("🗼 Paris (Eiffel)").clicked() {
                    geo.latitude = 48.8584;
                    geo.longitude = 2.2945;
                }
                if ui.button("🗽 New York").clicked() {
                    geo.latitude = 40.7128;
                    geo.longitude = -74.0060;
                }
                if ui.button("🗼 Tokyo Tower").clicked() {
                    geo.latitude = 35.6586;
                    geo.longitude = 139.7454;
                }
                if ui.button("🏛️ Rome").clicked() {
                    geo.latitude = 41.9028;
                    geo.longitude = 12.4964;
                }
            });
        }
        ContentKind::Crypto => {
            ui.label(egui::RichText::new("Cryptocurrency").strong());
            ui.horizontal(|ui| {
                ui.selectable_value(&mut crypto.currency, CryptoCurrency::Bitcoin, "₿ Bitcoin");
                ui.selectable_value(&mut crypto.currency, CryptoCurrency::Ethereum, "Ξ Ethereum");
                ui.selectable_value(&mut crypto.currency, CryptoCurrency::Solana, "◎ Solana");
                ui.selectable_value(&mut crypto.currency, CryptoCurrency::Usdt, "₮ USDT");
            });
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Wallet Address").strong());
            ui.add(
                egui::TextEdit::singleline(&mut crypto.address)
                    .hint_text("Public receiving address")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            ui.label(egui::RichText::new("Amount (optional)").strong());
            ui.add(
                egui::TextEdit::singleline(&mut crypto.amount)
                    .hint_text("e.g. 0.05")
                    .desired_width(f32::INFINITY),
            );
        }
    }
}
