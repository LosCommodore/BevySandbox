use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_plot::{Line, Plot, PlotPoints};
#[derive(Default)]
pub struct MyValue {
    v: f64,
}

#[derive(Resource, Default)]
pub struct EventHistory {
    pub v_small: Vec<f32>,
    pub x_small: Vec<f32>,
    pub v_big: Vec<f32>,
    pub x_big: Vec<f32>,
}

pub fn gui_system(
    mut contexts: EguiContexts,
    mut local: Local<MyValue>,
    event_history: ResMut<EventHistory>,
) -> Result<()> {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        // Beispiel-Daten generieren
        ui.label("world");
        ui.add(egui::widgets::DragValue::new(&mut local.v));

        let n = 128;
        let line_points: PlotPoints = (0..n)
            .map(|i| {
                let x = i as f64 * 0.1;
                [x, x.sin()]
            })
            .collect();

        let points: PlotPoints = (0..event_history.v_small.len())
            .map(|i| {
                [
                    event_history.v_big[i] as f64,
                    event_history.v_small[i] as f64,
                ]
            })
            .collect();

        // Plot Widget anzeigen
        Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| {
            plot_ui.line(Line::new("my_line", points));
        });
        //ui.label("world");
    });
    Ok(())
}
