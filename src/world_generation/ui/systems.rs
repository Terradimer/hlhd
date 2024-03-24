<<<<<<< HEAD
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
=======
use crate::{
    input::resources::Inputs,
    world_generation::{
        events::{LoadRoomEvent, SaveRoomEvent},
        ui::components::DebugButton,
    },
};
use bevy::{core_pipeline::core_2d::graph::input, prelude::*, utils::tracing::event};
use bevy_ecs::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use rfd::FileDialog;

pub fn setup_buttons(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        //border: UiRect::,
        ..default()
    };

    commands
        .spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: BackgroundColor::from(Color::BLACK.with_a(0.4)),
                border_color: BorderColor::from(Color::BLACK.with_a(0.5)),
                ..default()
            },
            DebugButton::Save,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Save Level",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            ButtonBundle {
                style: button_style,
                background_color: BackgroundColor::from(Color::BLACK.with_a(0.4)),
                border_color: BorderColor::from(Color::BLACK.with_a(0.5)),
                ..default()
            },
            DebugButton::Load,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Load Level",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
}

pub fn cleanup_dev_buttons(mut commands: Commands, query: Query<Entity, With<DebugButton>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn save_level_on_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DebugButton),
        Changed<Interaction>,
    >,
    mut ev_savelevel: EventWriter<SaveRoomEvent>,
    mut ev_loadlevel: EventWriter<LoadRoomEvent>,
) {
    for (interaction, mut color, event) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = BackgroundColor::from(Color::BLACK.with_a(0.4));
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::GRAY.with_a(0.4));
            }
            Interaction::Pressed => match event {
                DebugButton::Save => {
                    ev_savelevel.send(SaveRoomEvent);
                }
                DebugButton::Load => {
                    let load_path = FileDialog::new()
                        .add_filter("RON file", &["ron"])
                        .pick_file();

                    ev_loadlevel.send(LoadRoomEvent { path: load_path });
                }
            },
        }
>>>>>>> 4c83d017ac171220640e6d3480154aacf6ddb69a
    }
}