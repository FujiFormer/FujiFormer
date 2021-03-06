use bevy::prelude::*;
use fujiformer_io::Filler;

use super::{ui::MapUiResources, Map};

#[derive(Bundle)]
pub struct FillerBundle {
    filler: Filler,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

pub fn add_on_map_load(
    mut commands: Commands,
    map_create: Query<&Map, Added<Map>>,
    res: Res<MapUiResources>,
) {
    if let Some(map) = map_create.iter().next() {
        for filler in map.0.fillers().iter() {
            let rect = filler.shape();
            let size = rect.size();
            let width = size.width() as f32 * 8.0;
            let height = size.height() as f32 * 8.0;
            commands.spawn_bundle(FillerBundle {
                filler: filler.clone(),
                sprite_bundle: SpriteBundle {
                    sprite: Sprite::new(Vec2::new(width, height)),
                    material: res.filler_color.clone(),
                    transform: {
                        let pos = rect.position();
                        Transform::from_xyz(
                            (pos.x() * 8) as f32 + width * 0.5,
                            (pos.y() * 8) as f32 + height * 0.5,
                            0.0,
                        )
                    },
                    ..Default::default()
                },
            });
        }
    }
}
