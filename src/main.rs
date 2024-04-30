use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::input::gamepad::GamepadContext;
use ggez::{Context, GameError, GameResult};

struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {})
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the screen
        graphics::clear(ctx, Color::WHITE);

        // Draw a rectangle at position (100, 100) with width and height of 50
        let square = Rect::new(100.0, 100.0, 50.0, 50.0);
        let color = Color::new(0.0, 0.0, 1.0, 1.0); // Blue color
        let rectangle = MeshBuilder::new()
            .rectangle(DrawMode::fill(), square, color)
            .unwrap() // Unwrap the Result to get the MeshBuilder instance
            .build(ctx)?; // Build the mesh
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

        // Present the drawn graphics to the screen
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    // Create a new ggez context
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("graph_plotter", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Graph Plotter"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    // Create a new instance of your MainState struct
    let state = MainState::new(&mut ctx)?;

    // Run the event loop
    event::run(ctx, event_loop, state)
}
