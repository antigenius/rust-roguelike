mod components;
mod map;
mod player;
mod rect;

use rltk::{GameState, RGB, Rltk};
use specs::prelude::*;

use components::*;
use map::*;
use player::*;
use rect::*;

struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gamestate = State { ecs: World::new() };
    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();

    gamestate.ecs.insert(map);

    let (player_x, player_y) = rooms[0].center();

    gamestate
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y
        })
        .with(Renderable {
            bg: RGB::named(rltk::BLACK),
            fg: RGB::named(rltk::YELLOW),
            glyph: rltk::to_cp437('@'),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gamestate)
}
