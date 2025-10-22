use egui::{CentralPanel, Frame, Margin, SidePanel, Vec2};

use crate::{
    logic::truth_table::TruthTable,
    ui::{
        components::{
            map_view,
            menubar::{self, WindowState},
            properties_view, table_view, variable_view,
        }, 
        events::{self, EventQueue}, 
        modals, 
        variable::*
    },
};

#[derive(Default)]
pub struct AppState {
    pub modals: modals::ModalState,
    pub window_state: WindowState,
    pub variables: VariableStore,

    pub table: TruthTable,
    pub events: EventQueue
}

impl AppState {}

pub fn app() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui",
        native_options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let events = self.events.take_all();
        events::dispatch_all(self, events);
        
        menubar::update(ctx, &mut self.window_state);
        modals::update(ctx, self);

        SidePanel::left("left_panel")
            .resizable(true)
            .width_range(180.0..=720.0)
            .default_width(280.0)
            .show(ctx, |ui| {
                variable_view::render(ui, self);
                ui.separator();
                properties_view::render(ui, self);
            });

        // TODO: Better split pane
        // Maybe implement egui-dock in the future
        CentralPanel::default()
            .frame(Frame::new().inner_margin(Margin::ZERO))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                ui.spacing_mut().window_margin = Margin::ZERO;

                let frame = Frame::new()
                    .inner_margin(Margin::ZERO)
                    .fill(ui.visuals().faint_bg_color);

                // Left
                if self.window_state.map_view {
                    SidePanel::left("map_view")
                        .resizable(true)
                        .frame(frame)
                        .default_width(ui.available_width() / 2.0)
                        .show_inside(ui, |ui| {
                            map_view::render(ui);
                        });
                }

                // Right
                if self.window_state.table_view {
                    CentralPanel::default().frame(frame).show_inside(ui, |ui| {
                        table_view::render(ui, &mut self.table);
                    });
                }
            });
    }
}
