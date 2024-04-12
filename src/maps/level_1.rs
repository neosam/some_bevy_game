use bevy::ecs::component::Component;

use crate::map_builder;

#[derive(Component, Clone, Copy)]
pub struct NoTrigger;

fn level1_tile_mapper(c: char) -> Option<map_builder::TileType<NoTrigger>> {
    match c {
        'X' => Some(map_builder::TileType::Wall),
        'O' => Some(map_builder::TileType::Rock),
        _ => None,
    }
}

pub fn build_level_1() -> Result<map_builder::Map<NoTrigger>, map_builder::MapDraftError> {
    let map = [
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXX ",
        "X                          X ",
        "X                          X ",
        "X                          X ",
        "X   X                      X ",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "X   X                       X",
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    ];
    let draft = map_builder::MapDraft::from_str_array(&map, Box::new(level1_tile_mapper))?;

    Ok(draft.to_map((15, 12)))
}
