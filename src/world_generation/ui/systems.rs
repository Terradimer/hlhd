use crate::world_generation::ui::components::SaveLevelButtonTag;
use bevy::prelude::*;
use bevy_ecs::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn setup_dev_button(mut commands: Commands) {
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
                style: button_style,
                background_color: BackgroundColor::from(Color::BLACK.with_a(0.4)),
                border_color: BorderColor::from(Color::BLACK.with_a(0.5)),
                ..default()
            },
            SaveLevelButtonTag,
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
}

pub fn cleanup_dev_button(mut commands: Commands, query: Query<Entity, With<SaveLevelButtonTag>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn save_level_on_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SaveLevelButtonTag>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *color = BackgroundColor::from(Color::BLACK.with_a(0.4));
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(Color::GRAY.with_a(0.4));
            }
            Interaction::Pressed => {
                crate::world_generation::ui::functions::save_level();
            }
        }
    }
}

pub fn egui_ui_test(mut contexts: EguiContexts) {
    egui::Area::new("save button").fixed_pos((0.0, 0.0)).show(contexts.ctx_mut(), |ui| {
        for (_text_style, font_id) in ui.style_mut().text_styles.iter_mut() {
            font_id.size = 48.0 // whatever size you want here
        }
        if ui.button("Save").clicked() {
            crate::world_generation::ui::functions::save_level();
        }
    });
}