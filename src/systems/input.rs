use std::ops::Mul;

use bevy::{math::vec2, prelude::*, render::camera::Camera};
use crate::{NewGameEvent, Player, Tank, Velocity, Turret};

pub fn input_system(mut camera:Query<(&Camera,&Transform)>, mouse_button_input:Res<Input<MouseButton>>, mut mouse_moved_events:EventReader<CursorMoved>, keyboard_input:Res<Input<KeyCode>>, mut new_game:EventWriter<NewGameEvent>, mut player:Query<(&Player, &Tank, &mut Velocity, &Children)>, mut turrets:Query<(&mut Turret)>, windows:Res<Windows>) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_player, _, mut thrust, children)) = player.single_mut() {
        
        // tank movement input
        let speed = 2.0;
        let mut v = Vec3::default();
        if keyboard_input.pressed(KeyCode::W) {
            v.y = 1.0;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            v.y = -1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            v.x = -1.0;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            v.x = 1.0;
        }

        if v.length() > 0.0 {
            v = v.normalize() * speed;
        }
        thrust.velocity = v;

        // turret input
        for e in children.iter() {
            if let Ok(mut turret) = turrets.get_component_mut::<Turret>(*e) {
                turret.trigger = mouse_button_input.pressed(MouseButton::Left);
                turret.trigger = true;

                if let Ok((camera, transform)) = camera.single() {
                    for e in mouse_moved_events.iter() {
                        let wnd = windows.get_primary().expect("could not get primary monitor");
                        let wnd_size = Vec2::new(wnd.width(), wnd.height());
                        let ndc_pos = e.position / wnd_size * 2.0 - Vec2::new(1.0, 1.0);

                        let projection_matrix = camera.projection_matrix;
                        let transform_matrix = transform.compute_matrix();

                        let pos_world = (transform_matrix * (projection_matrix.inverse() * ndc_pos.extend(0.0).extend(1.0))).truncate();
                        println!("{:?}", pos_world);
                    }
                }
            }
        }


    }
}