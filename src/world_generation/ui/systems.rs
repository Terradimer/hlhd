use crate::world_generation::{rooms::events::*, ui::components::DebugButton};
use bevy::prelude::*;
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
    }
}
