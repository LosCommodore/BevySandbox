# Integrate egui_plot in Bevy

## Dependencies
Add to `Cargo.toml`:
```toml
bevy = "0.13"
bevy_egui = "0.27"
egui_plot = "0.27"
```

## Setup
```rust
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, plot_system)
        .run();
}
```

## Plot System
```rust
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Line, Plot, PlotPoints};

fn plot_system(mut contexts: EguiContexts) {
    egui::Window::new("Plot").show(contexts.ctx_mut(), |ui| {
        let sin: PlotPoints = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        }).collect();
        
        let line = Line::new(sin);
        
        Plot::new("plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    });
}
```

## Notes
- Replace sample data with actual data
- Check version compatibility
- Test performance with large datasets