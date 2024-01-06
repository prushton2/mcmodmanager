// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Ui;

fn main() -> Result<(), eframe::Error> {
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "MC Mod Manager",
        options,
        Box::new(|_cc| Box::<UserInterface>::default()),
    )

}

#[derive(Default)]
struct UserInterface {
    state: i64,
    // show_confirmation_dialog: bool,
    // allowed_to_close: bool,
}

impl eframe::App for UserInterface {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
        
            match self.state {
                10 => {
                    self.state = 0;
                },
                _ => {}
            }
            
            // ui.heading(format!("State is {}", self.state));
            
            main_menu(ui, self);
        
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Next").clicked() {
                        self.state += 1;
                    }
                    if ui.button("Back").clicked() {
                        self.state -= 1;
                    }
                    
                })

            })

            
            // ui.heading("Try to close the window");
            
        });
    
    }
}

fn main_menu(ui: &mut Ui, state: &mut UserInterface) {
    ui.heading(format!("State is {}", state.state));
}