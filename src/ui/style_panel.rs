use eframe::egui::{self, Color32, Ui};
use crate::types::{EcLevelSetting, EyeShape, ModuleShape, StyleConfig, PALETTES};

pub fn render_style_panel(ui: &mut Ui, style: &mut StyleConfig) {
    ui.collapsing("🎨 Color Palette & Themes", |ui| {
        ui.label(egui::RichText::new("Preset Themes:").small().strong());
        ui.horizontal_wrapped(|ui| {
            for preset in PALETTES {
                let is_selected = style.fg_color == preset.fg && style.bg_color == preset.bg;
                let btn = egui::Button::new(
                    egui::RichText::new(preset.name)
                        .color(preset.fg)
                        .strong(),
                )
                .fill(preset.bg)
                .stroke(if is_selected {
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(59, 130, 246))
                } else {
                    egui::Stroke::new(1.0, egui::Color32::from_gray(180))
                });

                if ui.add(btn).clicked() {
                    style.fg_color = preset.fg;
                    style.bg_color = preset.bg;
                    style.transparent_bg = false;
                }
            }
        });

        ui.add_space(8.0);
        ui.label(egui::RichText::new("Custom Colors:").small().strong());
        egui::Grid::new("color_picker_grid")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .show(ui, |ui| {
                ui.label("Foreground Color:");
                let mut fg_rgb = [style.fg_color.r(), style.fg_color.g(), style.fg_color.b()];
                if ui.color_edit_button_srgb(&mut fg_rgb).changed() {
                    style.fg_color = Color32::from_rgb(fg_rgb[0], fg_rgb[1], fg_rgb[2]);
                }
                ui.end_row();

                ui.label("Background Color:");
                ui.horizontal(|ui| {
                    let mut bg_rgb = [style.bg_color.r(), style.bg_color.g(), style.bg_color.b()];
                    if ui.color_edit_button_srgb(&mut bg_rgb).changed() {
                        style.bg_color = Color32::from_rgb(bg_rgb[0], bg_rgb[1], bg_rgb[2]);
                    }
                    ui.checkbox(&mut style.transparent_bg, "Transparent");
                });
                ui.end_row();
            });
    });

    ui.add_space(8.0);
    ui.collapsing("📐 Shape & Layout", |ui| {
        ui.label(egui::RichText::new("Module Dot Style:").small().strong());
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut style.module_shape, ModuleShape::Square, ModuleShape::Square.label());
            ui.selectable_value(&mut style.module_shape, ModuleShape::Rounded, ModuleShape::Rounded.label());
            ui.selectable_value(&mut style.module_shape, ModuleShape::Circle, ModuleShape::Circle.label());
            ui.selectable_value(&mut style.module_shape, ModuleShape::SoftSquare, ModuleShape::SoftSquare.label());
        });

        ui.add_space(6.0);
        ui.label(egui::RichText::new("Corner Eyes (Finder Patterns):").small().strong());
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut style.eye_shape, EyeShape::Unified, EyeShape::Unified.label());
            ui.selectable_value(&mut style.eye_shape, EyeShape::RoundedEye, EyeShape::RoundedEye.label());
            ui.selectable_value(&mut style.eye_shape, EyeShape::CircleEye, EyeShape::CircleEye.label());
        });

        ui.add_space(6.0);
        ui.label(egui::RichText::new("Quiet Zone / Padding (Modules):").small().strong());
        ui.add(egui::Slider::new(&mut style.margin, 0..=8).text("modules"));
    });

    ui.add_space(8.0);
    ui.collapsing("🛡️ Error Correction & Export Settings", |ui| {
        ui.label(egui::RichText::new("Error Correction Level:").small().strong());
        ui.horizontal(|ui| {
            ui.selectable_value(&mut style.ec_level, EcLevelSetting::Low, EcLevelSetting::Low.label());
            ui.selectable_value(&mut style.ec_level, EcLevelSetting::Medium, EcLevelSetting::Medium.label());
            ui.selectable_value(&mut style.ec_level, EcLevelSetting::Quartile, EcLevelSetting::Quartile.label());
            ui.selectable_value(&mut style.ec_level, EcLevelSetting::High, EcLevelSetting::High.label());
        });
        ui.label(
            egui::RichText::new(style.ec_level.description())
                .weak()
                .small(),
        );

        ui.add_space(6.0);
        ui.label(egui::RichText::new("Default Export Resolution (PNG):").small().strong());
        ui.horizontal(|ui| {
            for &res in &[512, 1024, 2048, 4096] {
                ui.selectable_value(&mut style.export_size, res, format!("{}px", res));
            }
        });
    });
}
