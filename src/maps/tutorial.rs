use crate::{
    map_builder,
    ship::{self, TutorialTrigger},
};

fn tutorial_tile_mapper(c: char) -> Option<map_builder::TileType<ship::TutorialTrigger>> {
    match c {
        'X' => Some(map_builder::TileType::Wall),
        'O' => Some(map_builder::TileType::Rock),
        '1' => Some(map_builder::TileType::SingleTrigger(
            TutorialTrigger::SimplyForward,
            1.1,
        )),
        '2' => Some(map_builder::TileType::SingleTrigger(
            TutorialTrigger::TurnedRight,
            1.1,
        )),
        '3' => Some(map_builder::TileType::SingleTrigger(
            TutorialTrigger::DeepSpace,
            1.1,
        )),
        _ => None,
    }
}

pub fn build_tutorial() -> Result<map_builder::Map<TutorialTrigger>, map_builder::MapDraftError> {
    let map = "XXXXXXXXXXXXXXXXXXXXXXXXXXXX \
                     X                          X \
                     X 1                      2 X \
                     X   XXXXXXXXXXXXXXXXXXXXX  X \
                     X   X                  XX  X \
                     X   X                  X   XX\
                     X   X                  X   O3\
                     X   X                  X   O \
                     X   X                  X   XX\
                     X   X                  XXXXX \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     X   X                        \
                     XXXXX                        ";
    let draft = map_builder::MapDraft::from_str(map, 29, 25, Box::new(tutorial_tile_mapper))?;

    Ok(draft.to_map((2, 2)))
}
