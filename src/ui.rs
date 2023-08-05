use eframe::egui;
use std::collections::BTreeSet;
use crate::cpu::CPU;

use std::rc::Rc;


pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
    }
}

pub fn ui(cpu: Box<CPU>) -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1920.0, 1080.0)),
        ..Default::default()
    };

    eframe::run_native(
        "runes", 
        options, 
        Box::new(|cc| Box::<MemoryInspector>::new(MemoryInspector::new(cc, cpu))))
}

struct Windows {
    windows: Vec<Box<dyn Window>>,
    open: BTreeSet<String>
}

struct MemoryInspector {
    enabled: bool,
    cpu_vram: [u8; 2048],
    page: u16,
}


impl MemoryInspector {
    pub fn new(cc: &eframe::CreationContext, cpu: Box<CPU>) -> Self {
        Self {
            enabled: true,
            cpu_vram: cpu.bus.cpu_vram,
            page: 0,
        }
    }
}


impl eframe::App for MemoryInspector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("CPU Memory Inspector").show(ctx, |ui| {
            // change style to monospace
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            // page selector
            ui.horizontal(|ui| {
                ui.label("Page: ");
                ui.add(egui::DragValue::new(&mut self.page).speed(1.0).clamp_range(0..=7));
            });

            for addr in 0..=15 {
                ui.horizontal(|ui| {
                    ui.label(format!("{:02X}{:2X}0", self.page, addr));
                    ui.separator();
                    for i in 0..16 {
                        // format as hex
                        ui.label(format!("{:02X}", self.cpu_vram[(self.page << 8 | addr << 4 | i) as usize]));
                    }
                });
            }
        });
    }
}

impl Window for MemoryInspector {
    fn name(&self) -> &'static str {
        "CPU Memory Inspector"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name()).show(ctx, |ui| {
            // change style to monospace
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

            // page selector
            ui.horizontal(|ui| {
                ui.label("Page: ");
                ui.add(egui::DragValue::new(&mut self.page).speed(1.0).clamp_range(0..=7));
            });

            for addr in 0..=15 {
                ui.horizontal(|ui| {
                    ui.label(format!("{:02X}{:2X}0", self.page, addr));
                    ui.separator();
                    for i in 0..16 {
                        // format as hex
                        ui.label(format!("{:02X}", self.cpu_vram[(self.page << 8 | addr << 4 | i) as usize]));
                    }
                });
            }
        });
    }
}
