mod map;
mod tile;
mod player;
mod components;
use rltk::*;
use specs::prelude::*;
use std::cmp::{max, min};
use std::borrow::{Borrow};
use map::*;
use tile::*;
use player::*;
use components::*;

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(0, 0, ctx.fps);

        self.run_systems();
        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let mut map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&mut map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::BError {
    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.insert(new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    rltk::main_loop(context, gs)
}
