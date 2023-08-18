use eframe::egui;
use crate::cpu::CPU;
use crate::ppu::SYSTEM_PALLETE;
use egui_dock::{DockArea, NodeIndex, Style, Tree};

use crate::opcodes::references;
use crate::renderer;

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

    chr_rom_texture: Option<egui::TextureHandle>,
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
            "ROM Header Inspector" => self.rom_header_inspector(ui),
            "CHR ROM Inspector" => self.chr_rom_inspector(ui),
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
            ui.add(egui::DragValue::new(&mut self.page_cpu).speed(1.0).clamp_range(0..=0x07));
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
                    ui.label(format!("{:02X}", self.cpu.bus.read_prg_rom(self.page_rom << 8 | addr << 4 | i)));
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
        ui.label(format!("Opcode {}", references::INSTRUCTION_LOOKUP[self.cpu.opcode as usize]));       
        ui.label(format!("Cycles: {:?}", self.cpu.cycles));
    }

    fn rom_header_inspector(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("PRG ROM Size: {}", self.cpu.bus.cartridge.header.prg_rom_size));
        ui.label(format!("CHR ROM Size: {}", self.cpu.bus.cartridge.header.chr_rom_size));
        ui.label(format!("Mapper: {}", (self.cpu.bus.cartridge.header.mapper_2 & 0xF0) | (self.cpu.bus.cartridge.header.mapper_1 >> 4)));
    }

    fn chr_rom_inspector(&mut self, ui: &mut egui::Ui) {
        let width = 256;
        let height = 240;
        let mut renderer = renderer::PPURenderer::new_custom_size(width, height);

        let mut tile_y = 0;
        let mut tile_x = 0;

        for tile_n in 0..255 {
            if tile_n != 0 && tile_n % 20 == 0 {
                tile_y += 10;
                tile_x = 0;
                
            }
            // load tiles into texture
            let tile = &self.cpu.bus.ppu.chr_rom[tile_n * 16 ..= tile_n * 16 + 15];

            for tile_index_y in 0..=7 {
                let mut upper = tile[tile_index_y];
                let mut lower = tile[tile_index_y + 8];

                for tile_index_x in (0..=7).rev() {
                    let color = (1 & upper) << 1 | (1 & lower);
                    upper >>= 1;
                    lower >>= 1;


                    let rgb = match color {
                        0 => SYSTEM_PALLETE[0x01],
                        1 => SYSTEM_PALLETE[0x23],
                        2 => SYSTEM_PALLETE[0x30],
                        3 => SYSTEM_PALLETE[0x3F],
                        _ => panic!("Invalid color value"),
                    };

                    renderer.set_pixel(tile_x + tile_index_x, tile_y + tile_index_y, rgb);
                }
            }

            tile_x += 10;
        }

        let texture: &egui::TextureHandle = self.chr_rom_texture.insert(
            ui.ctx().load_texture("chr-rom-texture", renderer.get_color_image(), Default::default()));
        
        ui.image(texture, [ui.available_size().min_elem(), ui.available_size().min_elem()]);


        
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

        let [game_node_index , cpu_memory_inspector_node_index] = tree.split_right(NodeIndex::root(), 0.78 ,vec!["CPU Memory Inspector".to_owned()]);

        tree.split_left(game_node_index, 0.3, vec!["CHR ROM Inspector".to_owned()]);

        let [_ , rom_memory_inspector_node_index] = tree.split_below(cpu_memory_inspector_node_index, 0.38, vec!["ROM Memory Inspector".to_owned(), "ROM Header Inspector".to_owned()]);
        let [_ , cpu_register_inspector_node_index] = tree.split_below(rom_memory_inspector_node_index, 0.7, vec!["CPU Register Inspector".to_owned()]);


        tree.split_right(cpu_register_inspector_node_index, 0.5, vec!["CPU Debug Inspector".to_owned()]);

        Self {
            context: RunesContext {
                cpu,
                page_cpu: 0,
                page_rom: 0x80,
                chr_rom_texture: None,
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


