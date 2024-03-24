use crate::world_generation::components::Focused;
use crate::world_generation::{MIN_SCALE, SNAP_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

fn scrollable_snapping_drag_value<'a>(ui: &mut egui::Ui, value: &'a mut f32, snapping_interval: f32)
{
    let response = ui.add(egui::DragValue::new(value).speed(snapping_interval));
    if response.hovered() {
        *value += ui.ctx().input(|i| i.raw_scroll_delta.normalized().y * snapping_interval);
    }
}

pub fn egui_ui_test(mut contexts: EguiContexts, mut query: Query<&mut Transform, With<Focused>>) {
    egui::Window::new("dev ui")
    .fixed_pos((0.0, 0.0))
    .collapsible(false)
    .fixed_size((WINDOW_WIDTH / 5.0, WINDOW_HEIGHT / 5.0))
    .show(contexts.ctx_mut(), |ui| {
        egui::CollapsingHeader::new("Save/Load")
        .default_open(true)
        .show(ui, |ui| {
            for (_text_style, font_id) in ui.style_mut().text_styles.iter_mut() {
                font_id.size = 48.0 // whatever size you want here
            }
            if ui.button("Save").clicked() {
                crate::world_generation::ui::functions::save_level();
            }
        });
        egui::CollapsingHeader::new("Properties Editor")
        .default_open(true)
        .show(ui, |ui| {

        });
    });
    
    //dbg!(query.iter().collect::<Vec<_>>());
    if let Ok(mut transform) = query.get_single_mut() {
        egui::Window::new("transform edit").fixed_pos((0.0, 100.0)).collapsible(false).resizable(false).show(contexts.ctx_mut(), move |ui| {
            let Vec3 {mut x, mut y, mut z} = transform.translation;

            ui.horizontal(|ui| {ui.label("X Translation:"); scrollable_snapping_drag_value(ui, &mut x, SNAP_SCALE)});
            ui.horizontal(|ui| {ui.label("Y Translation:"); scrollable_snapping_drag_value(ui, &mut y, SNAP_SCALE)});
            ui.horizontal(|ui| {ui.label("Z Translation:"); scrollable_snapping_drag_value(ui, &mut z, SNAP_SCALE)});

            transform.translation = Vec3 {x, y, z};
            
            let previous_scale = transform.scale;
            let Vec3 {mut x, mut y, z} = transform.scale;

            ui.horizontal(|ui| {ui.label("X Scale:"); scrollable_snapping_drag_value(ui, &mut x, SNAP_SCALE)});
            ui.horizontal(|ui| {ui.label("Y Scale:"); scrollable_snapping_drag_value(ui, &mut y, SNAP_SCALE)});

            let new_scale = Vec3 {x, y, z}.max(Vec3 {x: MIN_SCALE + MIN_SCALE / 2.0, y: MIN_SCALE + MIN_SCALE / 2.0, z});
            transform.scale = new_scale;
            transform.translation -= (previous_scale - new_scale) / Vec3::from_array([2.0, -2.0, 2.0]);
        });
    }
}