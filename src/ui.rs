use eframe::egui;
use crate::cpu::CPU;
use egui_dock::{DockArea, NodeIndex, Style, Tree};
use crate::opcodes::references;

pub fn ui(cpu: CPU) -> Result<(), eframe::Error> {
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
    cpu: CPU,
    page_cpu: u16,
    page_rom: u16,
}

impl egui_dock::TabViewer for RunesContext {
    type Tab = String;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "CPU Memory Inspector" => self.cpu_memory_inspector(ui),
            "Game" => self.game(ui),
            "CPU Register Inspector" => self.cpu_register_inspector(ui),
            "CPU Debug Inspector" => self.cpu_debug_inspector(ui),
            "ROM Memory Inspector" => self.rom_memory_inspector(ui),
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
            ui.add(egui::DragValue::new(&mut self.page_cpu).speed(1.0).clamp_range(0..=0xFF));
        });

        for addr in 0..=15 {
            ui.horizontal(|ui| {
                ui.label(format!("{:02X}{:2X}0", self.page_cpu, addr));
                ui.separator();
                for i in 0..=15 {
                    // format as hex
                    // only print when read from page 8000 ~ 8010
                    ui.label(format!("{:02X}", self.cpu.bus.cpu_vram[(self.page_cpu << 8 | addr << 4 | i) as usize]));
                }
            });
        }
    }

    fn rom_memory_inspector(&mut self, ui: &mut egui::Ui) {
        // change style to monospace
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

        // page selector
        ui.horizontal(|ui| {
            ui.label("Page: ");
            ui.add(egui::DragValue::new(&mut self.page_rom).speed(1.0).clamp_range(0x80..=0xFF));
        });

        for addr in 0..=15 {
            ui.horizontal(|ui| {
                ui.label(format!("{:02X}{:2X}0", self.page_rom, addr));
                ui.separator();
                for i in 0..=15 {
                    // format as hex
                    // only print when read from page 8000 ~ 8010
                    ui.label(format!("{:02X}", self.cpu.bus.cpu_vram[(self.page_rom << 8 | addr << 4 | i) as usize]));
                }
            });
        }
    }

    fn cpu_register_inspector(&mut self, ui: &mut egui::Ui) {
        // change style to monospace
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

        ui.horizontal(|ui| {
            ui.label("A: ");
            ui.label(format!("{:02X}", self.cpu.accumulator));
        });

        ui.horizontal(|ui| {
            ui.label("X: ");
            ui.label(format!("{:02X}", self.cpu.x_register));
        });

        ui.horizontal(|ui| {
            ui.label("Y: ");
            ui.label(format!("{:02X}", self.cpu.y_register));
        });

        ui.horizontal(|ui| {
            ui.label("PC: ");
            ui.label(format!("{:04X}", self.cpu.program_counter));
        });

        ui.horizontal(|ui| {
            ui.label("SP: ");
            ui.label(format!("{:02X}", self.cpu.stack_pointer));
        });

        ui.horizontal(|ui| {
            ui.label("Status: ");
            ui.label(format!("{:08b}", self.cpu.status));
        });

    }

    fn cpu_debug_inspector(&mut self, ui: &mut egui::Ui) {
        ui.label("CPU Debug Inspector");
        ui.label(format!("Opcode {}", references::INSTRUCTION_LOOKUP[self.cpu.opcode as usize]));       
        ui.label(format!("Cycles: {:?}", self.cpu.cycles));

    }

    fn game(&mut self, ui: &mut egui::Ui) {
        ui.label("Game");
    }
}

struct RunesApp {
    context: RunesContext,
    tree: Tree<String>
}


impl RunesApp {
    fn new(mut cpu: CPU) -> Self {
        let mut tree = Tree::new(vec!["Game".to_owned()]);

        let [_ , cpu_memory_inspector_node_index] = tree.split_right(NodeIndex::root(), 0.78 ,vec!["CPU Memory Inspector".to_owned()]);
        let [_ , rom_memory_inspector_node_index] = tree.split_below(cpu_memory_inspector_node_index, 0.38, vec!["ROM Memory Inspector".to_owned()]);
        let [_ , cpu_register_inspector_node_index] = tree.split_below(rom_memory_inspector_node_index, 0.7, vec!["CPU Register Inspector".to_owned()]);
        tree.split_right(cpu_register_inspector_node_index, 0.5, vec!["CPU Debug Inspector".to_owned()]);

        print!("initializing cpu...");

        // test code and cpu init
        let test_code = vec![
            0xA2, 0x0A,
            0x8E, 0x00, 0x00,
            0xA2, 0x03,
            0x8E, 0x01, 0x00,
            0xAC, 0x00, 0x00,
            0xA9, 0x00,
            0x18, 0x6D, 0x01, 0x00,
            0x88, 0xD0, 0xFA,
            0x8D, 0x02, 0x00,
            0xEA, 0xEA, 0xEA
        ];

        for (i, byte) in test_code.iter().enumerate() {
            cpu.bus.mem_write((0x8000 + i) as u16, *byte as u8);
        }

        cpu.bus.mem_write(0xFFFC, 0x00);
        cpu.bus.mem_write(0xFFFD, 0x80);

        cpu.reset();

        println!("done");

        Self {
            context: RunesContext {
                cpu,
                page_cpu: 0,
                page_rom: 0x80
            },
            tree
        }
    }
}

impl eframe::App for RunesApp { 
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.context);

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            loop {
                self.context.cpu.clock();
                if self.context.cpu.complete() {
                    break;
                }
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
        }
    }
}


