use eframe::egui::{self, Color32, Frame, Pos2, Rect, Rounding, Sense, Stroke, Vec2};
use eframe::egui::{ColorImage, ImageData};
use std::sync::Arc;
pub struct Frontend {}

impl Frontend {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Frontend {}
    }
}

impl eframe::App for Frontend {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Hello World!");
            });
        egui::SidePanel::left("my_left_panel")
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.label("Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!Hello World!");
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                ui.painter().rect_filled(
                    Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(100.0, 100.0)),
                    Rounding::same(0.0),
                    Color32::WHITE,
                );
                ctx.request_repaint();
            })
        });
    }
}
