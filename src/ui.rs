use eframe::egui;
use crate::cpu::CPU;
use egui_dock::{DockArea, NodeIndex, Style, Tree};

pub fn ui(cpu: Box<CPU>) -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1920.0, 1080.0)),
        ..Default::default()
    };

    eframe::run_native(
        "runes", 
        options, 
        Box::new(|_cc| Box::<RunesApp>::new(RunesApp::new(cpu))))
}

struct RunesContext {
    cpu_vram: [u8; 2048],
    page: u16,
}

impl egui_dock::TabViewer for RunesContext {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "CPU Memory Inspector" => self.cpu_memory_inspector(ui),
            "Game" => self.game(ui),
            _ => {}
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    
}

impl RunesContext {
    fn cpu_memory_inspector(&mut self, ui: &mut egui::Ui) {
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
    }

    fn game(&mut self, ui: &mut egui::Ui) {
        ui.label("Game");
    }
}

struct RunesApp {
    context: RunesContext,
    tree: Tree<String>
}

impl Default for RunesApp {
    fn default() -> Self {
        let mut tree = Tree::new(vec!["Game".to_owned()]);



        let context = RunesContext {
            cpu_vram: [0; 2048],
            page: 0,
        };

        tree.split_right(NodeIndex::root(), 0.78 ,vec!["CPU Memory Inspector".to_owned()]);
        Self {
            context,
            tree
        }
    }
}

impl RunesApp {
    fn new(cpu: Box<CPU>) -> Self {
        Self {
            context: RunesContext {
                cpu_vram: cpu.bus.cpu_vram,
                page: 0,
            },
            ..Default::default()
        }
    }
}

impl eframe::App for RunesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.context);
    }
}


