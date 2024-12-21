use archer_tower::SpawnArcherTower;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseSpriteAnimation, Aseprite};
use common::{ArenaPos, Direction, PlayerNumber, Unit, UnitState};
use king_tower::SpawnKingTower;

use crate::screens::GameState;

mod archer_tower;
mod king_tower;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Direction>();
    app.register_type::<UnitState>();

    app.add_systems(
        PreUpdate,
        manage_animation.run_if(in_state(GameState::Gameplay)),
    );

    app.add_plugins((archer_tower::plugin, king_tower::plugin));
}

fn manage_animation(
    mut animation_query: Query<(
        &Direction,
        &UnitState,
        &mut AseSpriteAnimation,
        &mut AnimationState,
    )>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for (direction, state, mut animation, mut animation_state) in animation_query.iter_mut() {
        match state {
            UnitState::Idle => {
                let tag_meta = aseprites
                    .get(animation.aseprite.id())
                    .unwrap()
                    .tags
                    .get(direction.tag())
                    .unwrap();
                let start_frame = tag_meta.range.start();
                animation_state.current_frame = *start_frame;

                animation.animation.tag = Some(direction.tag().into());
            }
            UnitState::Moving => {
                let tag_meta = aseprites
                    .get(animation.aseprite.id())
                    .unwrap()
                    .tags
                    .get(direction.tag())
                    .unwrap();
                let start_frame = tag_meta.range.start();
                let end_frame = tag_meta.range.end();
                if animation_state.current_frame < *start_frame
                    || animation_state.current_frame > *end_frame
                {
                    animation_state.current_frame = *start_frame;
                }

                animation.animation.tag = Some(direction.tag().into());
            }
            UnitState::Attacking => {
                let mut tag = String::from(direction.tag());
                tag.push('a');

                let tag_meta = aseprites
                    .get(animation.aseprite.id())
                    .unwrap()
                    .tags
                    .get(&tag)
                    .unwrap();
                let start_frame = tag_meta.range.start();
                let end_frame = tag_meta.range.end();
                if animation_state.current_frame < *start_frame
                    || animation_state.current_frame > *end_frame
                {
                    animation_state.current_frame = *start_frame;
                }

                animation.animation.tag = Some(tag);
            }
        }
    }
}

pub(super) trait Spawn {
    fn spawn(
        &self,
        entity: Entity,
        pos: ArenaPos,
        player_num: PlayerNumber,
        cmd: &mut Commands,
    );
}

impl Spawn for Unit {
    fn spawn(
        &self,
        entity: Entity,
        pos: ArenaPos,
        player_num: PlayerNumber,
        cmd: &mut Commands,
    ) {
        match self {
            Unit::ArcherTower => cmd.trigger(SpawnArcherTower(entity, pos, player_num)),
            Unit::KingTower => cmd.trigger(SpawnKingTower(entity, pos, player_num)),
        }
    }
}

trait SpawnDirection {
    fn spawn_direction(self, player_num: Self) -> Direction;
}

impl SpawnDirection for PlayerNumber {
    fn spawn_direction(self, player_num: PlayerNumber) -> Direction {
        use PlayerNumber::*;
        match (self, player_num) {
            (One, One) | (Two, Two) => Direction::Up,
            _ => Direction::Down,
        }
    }
}

trait IntoTag {
    fn tag(&self) -> &'static str;
}
impl IntoTag for Direction {
    fn tag(&self) -> &'static str {
        match self {
            Direction::Up => "u",
            Direction::Down => "d",
            Direction::Left => "l",
            Direction::Right => "r",
        }
    }
}
impl IntoTag for UnitState {
    fn tag(&self) -> &'static str {
        match self {
            UnitState::Idle => "",
            UnitState::Moving => "",
            UnitState::Attacking => "a",
        }
    }
}

trait SpawnPosition {
    fn spawn_pos(&self, pos: ArenaPos) -> ArenaPos;
}

impl SpawnPosition for PlayerNumber {
    fn spawn_pos(&self, pos: ArenaPos) -> ArenaPos {
        match self {
            PlayerNumber::One => pos,
            PlayerNumber::Two => ArenaPos(-pos.0, -pos.1),
        }
    }
}
