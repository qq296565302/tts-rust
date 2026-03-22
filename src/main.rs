mod config;
mod gui;
mod player;
mod tts;

use eframe::egui;
use std::fs;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("TTS语音生成工具"),
        ..Default::default()
    };

    eframe::run_native(
        "TTS语音生成工具",
        options,
        Box::new(|cc| {
            setup_light_theme(&cc.egui_ctx);
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(gui::TtsApp::new(cc))
        }),
    )
}

fn setup_light_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    style.visuals.window_fill = egui::Color32::from_rgb(245, 247, 250);
    style.visuals.panel_fill = egui::Color32::from_rgb(245, 247, 250);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(245, 247, 250);
    
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::WHITE;
    style.visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(245, 247, 250);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(236, 245, 255);
    
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(96, 98, 102);
    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(48, 49, 51);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(48, 49, 51);
    style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(64, 158, 255);
    
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 223, 230));
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 223, 230));
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(192, 196, 204));
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(64, 158, 255));
    
    style.visuals.selection.bg_fill = egui::Color32::from_rgb(64, 158, 255);
    style.visuals.selection.stroke.color = egui::Color32::WHITE;
    
    style.visuals.window_rounding = 4.0.into();
    style.visuals.menu_rounding = 4.0.into();
    
    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;
    
    ctx.set_style(style);
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    let font_paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];
    
    for font_path in font_paths {
        if let Ok(font_data) = fs::read(font_path) {
            fonts.font_data.insert(
                "chinese_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "chinese_font".to_owned());
            
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "chinese_font".to_owned());
            
            break;
        }
    }
    
    ctx.set_fonts(fonts);
}
