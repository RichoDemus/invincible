use bevy::prelude::*;

pub(crate) struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fonts>();
        app.init_resource::<Sprites>();
    }
}

#[derive(Resource)]
pub(crate) struct Fonts {
    pub(crate) font: Handle<Font>,
}

impl FromWorld for Fonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Fonts {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        }
    }
}

#[derive(Resource)]
pub(crate) struct Sprites {
    pub(crate) selection_box: Handle<Image>,
    // pub(crate) pawn: Handle<TextureAtlas>,
    // pub(crate) trees: Handle<TextureAtlas>,
    // pub(crate) grass: Handle<TextureAtlas>,
    // pub(crate) map: Handle<Texture>,
}

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        // let (lumberjack, map, grass, trees) = {
        //     let asset_server = world.get_resource::<AssetServer>().unwrap();
        //     (
        //         asset_server.load("lumberjack.png"),
        //         asset_server.load("map.png"),
        //         asset_server.load("grass.png"),
        //         asset_server.load("trees.png"),
        //     )
        // };
        // let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        // let lumberjack_atlas = TextureAtlas::from_grid(lumberjack, Vec2::new(32., 32.), 4, 3);
        // let lumberjack_atlas = texture_atlases.add(lumberjack_atlas);
        //
        // let tree_atlas = TextureAtlas::from_grid(trees, Vec2::new(32., 32.), 6, 3);
        // let tree_atlas = texture_atlases.add(tree_atlas);
        //
        // let grass_atlas = TextureAtlas::from_grid(grass, Vec2::new(32., 32.), 3, 4);
        // let grass_atlas = texture_atlases.add(grass_atlas);

        Sprites {
            // pawn: lumberjack_atlas,
            // trees: tree_atlas,
            // grass: grass_atlas,
            selection_box: asset_server.load("selection_box.png"),
        }
    }
}
