use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use crate::{Owner, Projectile};

use super::*;

pub fn thing_builder_added_system(mut commands:Commands, mut query:Query<(Entity, &ThingBuilder), Added<ThingBuilder>>, texture_atlases:Res<TextureAtlases>) {
    query.for_each_mut(|(e, tb)| {
        let mut e = commands.entity(e);
        let transform = Transform {
            translation:tb.translation,
            rotation:tb.rotation,
            scale:Vec3::splat(1.0 / 8.0)
        };
        
        let sprite_sheet_bundle = SpriteSheetBundle {
            texture_atlas:texture_atlases.get_atlas(tb.thing_type),
            transform,
            sprite:TextureAtlasSprite {
                index:texture_atlases.get_index(tb.thing_type),
                ..Default::default()
            },
            ..Default::default()
        };

        e.insert_bundle(sprite_sheet_bundle);

        if let Some(entity) = tb.owner {
            e.insert(Owner::from(entity));
        }

        let x = tb.translation.x;
        let y = tb.translation.y;

        match tb.thing_type {
            ThingType::Unknown => {}
            ThingType::Tank => {

            }
            ThingType::Bullet => {
                let speed = 10.0;
                let v = Vec3::new(speed, 0.0, 0.0);
                let v = tb.rotation * v;
                let a = v.angle_between(Vec3::new(1.0, 0.0, 0.0));
                let rigid_body = RigidBodyBuilder::new_dynamic()
                .translation(x, y)
                //.linear_damping(1.5)
                //.angular_damping(1.5)
                .linvel(v.x, v.y)
                .angvel(10.0)
                .rotation(a);

                e.insert(rigid_body);
                
                let collider = ColliderBuilder::cuboid(1.0/8.0, 1.0/8.0)
                .user_data(e.id().to_bits() as u128)
                .modify_solver_contacts(true);
                e.insert(collider);
                e.insert(Projectile::default());
            }

        }
    });
}

pub fn thing_builder_init_system(mut textures:ResMut<TextureAtlases>, asset_server:Res<AssetServer>, mut texture_atlases:ResMut<Assets<TextureAtlas>>) {
    textures.tanks = texture_atlases.add(TextureAtlas::from_grid(asset_server.load("tanks.png"), Vec2::new(8.0, 8.0), 4, 4));
}