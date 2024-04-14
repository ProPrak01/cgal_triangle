// src/main.rs
use eframe::egui::{self, Color32, Pos2, Shape, Stroke};
struct TestVal {
    point_a: Pos2,
    point_b: Pos2,
    point_c: Pos2,
    ccx: f32,
    ccy: f32,
}
impl Default for TestVal {
    fn default() -> Self {
        Self {
            point_a: Pos2::new(100.0, 400.0),
            point_b: Pos2::new(300.0, 300.0),
            point_c: Pos2::new(400.0, 400.0),
            ccx: 400.0,
            ccy: 650.0,
        }
    }
}

impl eframe::App for TestVal {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Triangle App");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Point A:");
                    ui.add(
                        egui::DragValue::new(&mut self.point_a.x)
                            .speed(1.0)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.point_a.y)
                            .speed(1.0)
                            .prefix("y: "),
                    );
                });
                ui.vertical(|ui| {
                    ui.label("Point B:");
                    ui.add(
                        egui::DragValue::new(&mut self.point_b.x)
                            .speed(1.0)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.point_b.y)
                            .speed(1.0)
                            .prefix("y: "),
                    );
                });
                ui.vertical(|ui| {
                    ui.label("Point C:");
                    ui.add(
                        egui::DragValue::new(&mut self.point_c.x)
                            .speed(1.0)
                            .prefix("x: "),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.point_c.y)
                            .speed(1.0)
                            .prefix("y: "),
                    );
                });
            });

            let area = unsafe {
                ffi::compute_area(
                    self.point_a.x.into(),
                    self.point_a.y.into(),
                    self.point_b.x.into(),
                    self.point_b.y.into(),
                    self.point_c.x.into(),
                    self.point_c.y.into(),
                )
            };

            let area_text = format!("Area: {:.2}", area);
            let centroid_text = format!("Centroid: x:{:.2} y:{:.2}", self.ccx, self.ccy);
            ui.label(area_text.clone());
            ui.label(centroid_text);
            let text_color = egui::Color32::BLUE;
            let text_position = egui::Pos2::new(self.ccx as f32, self.ccy + 20 as f32);
            let text_align = egui::Align2::CENTER_CENTER;
            let font_id = egui::FontId::default();

            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let points = vec![self.point_a, self.point_b, self.point_c, self.point_a];

            unsafe {
                ffi::compute_centroid(
                    self.point_a.x.into(),
                    self.point_a.y.into(),
                    self.point_b.x.into(),
                    self.point_b.y.into(),
                    self.point_c.x.into(),
                    self.point_c.y.into(),
                    &mut self.ccx,
                    &mut self.ccy,
                )
            };
            let stroke = egui::Stroke::new(2.0, Color32::BLACK);

            painter.add(Shape::line(points, stroke));
            painter.text(text_position, text_align, &area_text, font_id, text_color);

            painter.circle_filled(Pos2::new(self.ccx, self.ccy), 5.0, Color32::RED);
            painter.circle_filled(self.point_a, 5.0, Color32::GREEN);
            painter.circle_filled(self.point_b, 5.0, Color32::GREEN);
            painter.circle_filled(self.point_c, 5.0, Color32::GREEN);
        });
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cargoCGAL/include/triangle.h");
        unsafe fn compute_centroid(
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            x3: f64,
            y3: f64,
            cx: *mut f32,
            cy: *mut f32,
        );
        unsafe fn compute_area(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64;

    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "presenting triangle",
        options,
        Box::new(|_cc| Box::new(TestVal::default())),
    );
}
