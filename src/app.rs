use eframe::egui::{self, Color32, ColorImage, TextureOptions};
use std::fs;

use crate::generator::{
    build_payload, copy_image_to_clipboard, copy_text_to_clipboard, generate_matrix,
    render_image, render_svg, QrMatrix,
};
use crate::types::{
    ContentKind, CryptoData, EmailData, GeoData, SmsData, StyleConfig, ToastNotification,
    VCardData, WifiData,
};
use crate::ui::{
    content_form::render_content_form,
    preview_panel::{render_preview_panel, PreviewAction},
    style_panel::render_style_panel,
};

pub struct QrApp {
    // Content state
    pub active_tab: ContentKind,
    pub url: String,
    pub text: String,
    pub wifi: WifiData,
    pub vcard: VCardData,
    pub email: EmailData,
    pub phone: String,
    pub sms: SmsData,
    pub geo: GeoData,
    pub crypto: CryptoData,

    // Style
    pub style: StyleConfig,

    // Cache to avoid regenerating on every frame
    current_payload: String,
    matrix: Option<QrMatrix>,
    texture: Option<egui::TextureHandle>,
    last_render_key: String,
    error_message: Option<String>,

    // UI state
    pub dark_mode: bool,
    pub toast: Option<ToastNotification>,
}

impl Default for QrApp {
    fn default() -> Self {
        Self {
            active_tab: ContentKind::Url,
            url: "https://github.com".to_string(),
            text: "Hello from QRust!".to_string(),
            wifi: WifiData::default(),
            vcard: VCardData::default(),
            email: EmailData::default(),
            phone: "+1 555-0199".to_string(),
            sms: SmsData::default(),
            geo: GeoData::default(),
            crypto: CryptoData::default(),

            style: StyleConfig::default(),

            current_payload: String::new(),
            matrix: None,
            texture: None,
            last_render_key: String::new(),
            error_message: None,

            dark_mode: true,
            toast: None,
        }
    }
}

impl QrApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }

    fn show_toast(&mut self, ctx: &egui::Context, message: impl Into<String>, is_error: bool) {
        let now = ctx.input(|i| i.time);
        self.toast = Some(ToastNotification {
            message: message.into(),
            is_error,
            expiry_time: now + 3.5,
        });
    }

    fn compute_render_key(&self, payload: &str) -> String {
        format!(
            "{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
            payload,
            self.style.margin,
            self.style.module_shape,
            self.style.eye_shape,
            self.style.ec_level,
            self.style.transparent_bg,
            self.style.fg_color.to_array(),
            self.style.bg_color.to_array()
        )
    }

    fn update_qr_if_needed(&mut self, ctx: &egui::Context) {
        let payload = build_payload(
            self.active_tab,
            &self.url,
            &self.text,
            &self.wifi,
            &self.vcard,
            &self.email,
            &self.phone,
            &self.sms,
            &self.geo,
            &self.crypto,
        );

        let key = self.compute_render_key(&payload);
        if key == self.last_render_key {
            return;
        }

        self.last_render_key = key;
        self.current_payload = payload.clone();

        match generate_matrix(&payload, self.style.ec_level) {
            Ok(mat) => {
                self.error_message = None;
                // Render preview image at 384px for high-DPI crispness
                let img = render_image(&mat, &self.style, 384);
                let (w, h) = img.dimensions();
                let color_image = ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    img.as_raw(),
                );

                if let Some(tex) = &mut self.texture {
                    tex.set(color_image, TextureOptions::LINEAR);
                } else {
                    self.texture = Some(ctx.load_texture("qr_preview", color_image, TextureOptions::LINEAR));
                }

                self.matrix = Some(mat);
            }
            Err(e) => {
                self.error_message = Some(e);
                self.matrix = None;
                self.texture = None;
            }
        }
    }
}

impl eframe::App for QrApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.update_qr_if_needed(&ctx);

        // Top Navigation Bar
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("⬛ QRust")
                        .strong()
                        .size(19.0)
                        .color(Color32::from_rgb(96, 165, 250)),
                );
                ui.label(
                    egui::RichText::new("· Studio Generator")
                        .weak()
                        .size(14.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_icon = if self.dark_mode { "☀️ Light" } else { "🌙 Dark" };
                    if ui.button(theme_icon).clicked() {
                        self.dark_mode = !self.dark_mode;
                        if self.dark_mode {
                            ui.ctx().set_visuals(egui::Visuals::dark());
                        } else {
                            ui.ctx().set_visuals(egui::Visuals::light());
                        }
                    }

                    if ui.button("🔄 Reset").on_hover_text("Reset to defaults").clicked() {
                        *self = QrApp::default();
                    }
                });
            });
            ui.add_space(4.0);
        });

        // Bottom Status / Toast Bar
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let now = ui.ctx().input(|i| i.time);
                if let Some(toast) = &self.toast {
                    if now < toast.expiry_time {
                        let color = if toast.is_error {
                            Color32::from_rgb(239, 68, 68)
                        } else {
                            Color32::from_rgb(34, 197, 94)
                        };
                        ui.colored_label(color, &toast.message);
                    } else {
                        ui.weak("Ready");
                    }
                } else {
                    ui.weak("Ready");
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.weak("Rust + egui + qrcode");
                });
            });
            ui.add_space(4.0);
        });

        // Central Area with 2 Columns
        let mut triggered_action = None;

        egui::CentralPanel::default().show(ui, |ui| {
            ui.columns(2, |columns| {
                // LEFT COLUMN: Content & Customization
                let left_ui = &mut columns[0];
                egui::ScrollArea::vertical()
                    .id_salt("left_scroll")
                    .auto_shrink([false, false])
                    .show(left_ui, |ui| {
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("1. Choose Content Type").strong().size(15.0));
                        ui.add_space(4.0);

                        // Content Kind Selector
                        ui.horizontal_wrapped(|ui| {
                            let kinds = [
                                ContentKind::Url,
                                ContentKind::Text,
                                ContentKind::Wifi,
                                ContentKind::VCard,
                                ContentKind::Email,
                                ContentKind::Phone,
                                ContentKind::Sms,
                                ContentKind::Geo,
                                ContentKind::Crypto,
                            ];
                            for kind in kinds {
                                let is_active = self.active_tab == kind;
                                let btn = egui::Button::new(
                                    egui::RichText::new(kind.label()).strong()
                                )
                                .fill(if is_active {
                                    Color32::from_rgb(37, 99, 235)
                                } else {
                                    Color32::from_gray(if self.dark_mode { 45 } else { 220 })
                                })
                                .stroke(egui::Stroke::NONE);

                                if ui.add(btn).clicked() {
                                    self.active_tab = kind;
                                }
                            }
                        });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Form Inputs
                        render_content_form(
                            ui,
                            self.active_tab,
                            &mut self.url,
                            &mut self.text,
                            &mut self.wifi,
                            &mut self.vcard,
                            &mut self.email,
                            &mut self.phone,
                            &mut self.sms,
                            &mut self.geo,
                            &mut self.crypto,
                        );

                        ui.add_space(14.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Style Controls
                        ui.label(egui::RichText::new("2. Design & Styling").strong().size(15.0));
                        ui.add_space(6.0);
                        render_style_panel(ui, &mut self.style);
                        ui.add_space(16.0);
                    });

                // RIGHT COLUMN: Live QR Preview & Actions
                let right_ui = &mut columns[1];
                egui::ScrollArea::vertical()
                    .id_salt("right_scroll")
                    .auto_shrink([false, false])
                    .show(right_ui, |ui| {
                        ui.add_space(6.0);
                        triggered_action = render_preview_panel(
                            ui,
                            self.texture.as_ref(),
                            self.matrix.as_ref(),
                            &self.style,
                            &self.current_payload,
                            self.error_message.as_deref(),
                        );
                    });
            });
        });

        // Handle user action triggers
        if let Some(action) = triggered_action {
            if let Some(mat) = &self.matrix {
                match action {
                    PreviewAction::CopyImage => {
                        let img = render_image(mat, &self.style, self.style.export_size.min(1024));
                        match copy_image_to_clipboard(&img) {
                            Ok(()) => self.show_toast(&ctx, "✓ QR Code copied to system clipboard!", false),
                            Err(e) => self.show_toast(&ctx, format!("⚠️ Failed to copy: {}", e), true),
                        }
                    }
                    PreviewAction::CopyText => {
                        match copy_text_to_clipboard(&self.current_payload) {
                            Ok(()) => self.show_toast(&ctx, "✓ Content payload copied to clipboard!", false),
                            Err(e) => self.show_toast(&ctx, format!("⚠️ Failed to copy: {}", e), true),
                        }
                    }
                    PreviewAction::SavePng => {
                        let default_name = format!(
                            "qrcode_{}.png",
                            match self.active_tab {
                                ContentKind::Url => "url",
                                ContentKind::Wifi => "wifi",
                                ContentKind::VCard => "contact",
                                ContentKind::Email => "email",
                                _ => "code",
                            }
                        );

                        let file_path = rfd::FileDialog::new()
                            .add_filter("PNG Image (*.png)", &["png"])
                            .set_file_name(&default_name)
                            .save_file();

                        if let Some(path) = file_path {
                            let img = render_image(mat, &self.style, self.style.export_size);
                            match img.save(&path) {
                                Ok(()) => {
                                    self.show_toast(
                                        &ctx,
                                        format!("✓ Saved PNG to {}", path.file_name().unwrap_or_default().to_string_lossy()),
                                        false,
                                    );
                                }
                                Err(e) => {
                                    self.show_toast(&ctx, format!("⚠️ Failed to save image: {}", e), true);
                                }
                            }
                        }
                    }
                    PreviewAction::SaveSvg => {
                        let default_name = "qrcode.svg";
                        let file_path = rfd::FileDialog::new()
                            .add_filter("SVG Vector (*.svg)", &["svg"])
                            .set_file_name(default_name)
                            .save_file();

                        if let Some(path) = file_path {
                            let svg_content = render_svg(mat, &self.style);
                            match fs::write(&path, svg_content) {
                                Ok(()) => {
                                    self.show_toast(
                                        &ctx,
                                        format!("✓ Saved SVG to {}", path.file_name().unwrap_or_default().to_string_lossy()),
                                        false,
                                    );
                                }
                                Err(e) => {
                                    self.show_toast(&ctx, format!("⚠️ Failed to save SVG: {}", e), true);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Request repaint if toast notification is active to ensure smooth expiration
        if let Some(toast) = &self.toast {
            let now = ctx.input(|i| i.time);
            if now < toast.expiry_time {
                ctx.request_repaint_after(std::time::Duration::from_millis(200));
            }
        }
    }
}
