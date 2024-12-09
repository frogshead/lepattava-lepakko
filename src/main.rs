use bracket_lib::prelude::*;

struct State {
    mode: GameMode
}

enum GameMode {
    Manu,
    Playing,
    End
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Manu,
        } 
    }
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO:  Fill this stub later
        self.mode = GameMode::End;
        todo!();
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Tervetuloa pelaamaan 'lepattavaa lepakkoa'");
        ctx.print_centered(8, "(P) Pelaa!");
        ctx.print_centered(9, "(Q) Lopeta");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q  => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Kuolit! piip-piip!");
        ctx.print_centered(8, "(P) Haluatko pelata uudestaan? Paina P");
        ctx.print_centered(9, "(Q) Lopeta");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Manu => self.main_menu(ctx),
            GameMode::Playing => self.dead(ctx),
            GameMode::End => self.play(ctx),
        }
    }
}




fn main() -> BError {
    println!("Hello, world!");
    let context = BTermBuilder::simple80x50()
    .with_title("Lepattava lepakko")
    .build()?;
    main_loop(context, State::new())
}
