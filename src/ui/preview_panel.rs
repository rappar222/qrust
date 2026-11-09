use eframe::egui::{self, Color32, Pos2, Rect, Stroke, TextureHandle, Ui, Vec2};
use crate::generator::QrMatrix;
use crate::types::StyleConfig;

pub enum PreviewAction {
    CopyImage,
    CopyText,
    SavePng,
    SaveSvg,
}

pub fn render_preview_panel(
    ui: &mut Ui,
    texture: Option<&TextureHandle>,
    matrix: Option<&QrMatrix>,
    style: &StyleConfig,
    payload: &str,
    err_msg: Option<&str>,
) -> Option<PreviewAction> {
    let mut triggered_action = None;

    ui.vertical_centered(|ui| {
        ui.heading("Live Preview");
        ui.add_space(8.0);

        if let Some(err) = err_msg {
            ui.colored_label(Color32::from_rgb(239, 68, 68), format!("⚠️ {}", err));
            return;
        }

        // Draw preview container with checkerboard background
        let desired_size = Vec2::splat(320.0);
        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        let painter = ui.painter();

        // 1. Draw rounded outer frame
        painter.rect(
            rect,
            8.0,
            Color32::from_gray(30),
            Stroke::new(1.0, Color32::from_gray(60)),
            egui::StrokeKind::Inside,
        );

        // 2. Draw checkerboard pattern inside for transparency perception
        let check_size = 16.0;
        let c1 = Color32::from_gray(45);
        let c2 = Color32::from_gray(65);
        let qr_inner_rect = rect.shrink(12.0);

        let cols = (qr_inner_rect.width() / check_size).ceil() as usize;
        let rows = (qr_inner_rect.height() / check_size).ceil() as usize;

        for r in 0..rows {
            for c in 0..cols {
                let x = qr_inner_rect.min.x + c as f32 * check_size;
                let y = qr_inner_rect.min.y + r as f32 * check_size;
                let cell_rect = Rect::from_min_size(
                    Pos2::new(x, y),
                    Vec2::splat(check_size),
                ).intersect(qr_inner_rect);

                let color = if (r + c) % 2 == 0 { c1 } else { c2 };
                painter.rect_filled(cell_rect, 0.0, color);
            }
        }

        // 3. Draw the actual QR texture
        if let Some(tex) = texture {
            painter.image(
                tex.id(),
                qr_inner_rect,
                Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }

        ui.add_space(12.0);

        // Info chips
        if let Some(m) = matrix {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("📏 Matrix: {}×{}", m.width, m.width))
                        .small()
                        .monospace(),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("📊 Version: {:?}", m.version))
                        .small()
                        .monospace(),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(format!("📦 Size: {} bytes", payload.len()))
                        .small()
                        .monospace(),
                );
            });
        }

        ui.add_space(14.0);

        // Action Buttons
        ui.horizontal(|ui| {
            let copy_btn = egui::Button::new(
                egui::RichText::new("📋 Copy Image")
                    .strong()
                    .color(Color32::WHITE),
            )
            .fill(Color32::from_rgb(37, 99, 235)) // Blue 600
            .min_size(Vec2::new(120.0, 36.0));

            if ui.add(copy_btn).clicked() {
                triggered_action = Some(PreviewAction::CopyImage);
            }

            let save_png_btn = egui::Button::new(
                egui::RichText::new(format!("💾 Save PNG ({}px)", style.export_size))
                    .strong(),
            )
            .min_size(Vec2::new(130.0, 36.0));

            if ui.add(save_png_btn).clicked() {
                triggered_action = Some(PreviewAction::SavePng);
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            if ui.add(egui::Button::new("📐 Save SVG (Vector)").min_size(Vec2::new(130.0, 30.0))).clicked() {
                triggered_action = Some(PreviewAction::SaveSvg);
            }

            if ui.add(egui::Button::new("📄 Copy Raw Payload").min_size(Vec2::new(120.0, 30.0))).clicked() {
                triggered_action = Some(PreviewAction::CopyText);
            }
        });

        ui.add_space(10.0);
        ui.collapsing("🔍 Preview Raw Content", |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut payload.to_string())
                    .desired_width(f32::INFINITY)
                    .desired_rows(3)
                    .interactive(false),
            );
        });
    });

    triggered_action
}
