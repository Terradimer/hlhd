macro_rules! add_animation {
    ($path:expr, $num_frames:expr, $asset_server:expr, $texture_atlas:expr) => {{
        let handle = $asset_server.get_handle($path).unwrap();
        let index = $texture_atlas.get_texture_index(&handle).unwrap();

        crate::animation::components::Animation {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            indicies: AnimationIndices {
                first: index,
                last: index + $num_frames,
            },
        }
    }};
}

pub(crate) use add_animation;
