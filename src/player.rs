use specs::*;
use specs_derive::Component;
use crate::components::*;
use crate::tile::*;
use crate::map::*;
use rltk::*;
use crate::State;
use std::cmp::{min, max};

#[derive(Component)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Vec<TileType>>();

    for (_, pos) in (&players, &mut positions).join() {
        if map[xy_idx(pos.x + delta_x, pos.y + delta_y)] == TileType::Floor {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub(crate) fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}