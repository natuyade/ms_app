use bevy::prelude::*;
use crate::TitleModel;

pub fn title_rotate(
    time : Res<Time>,
    mut query: Query<&mut Transform, With<TitleModel>>
) {
    for mut transform in query {
        //transform.rotate_y(time.delta_secs() * 0.1);
    }
}