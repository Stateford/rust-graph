extern crate piston_window;
mod graph;

use self::graph::instance::Graph;
use piston_window::*;
use piston_window::Event::Loop;
use piston_window::Loop::Render;
use piston_window::Loop::Update;

struct MainWindow {
    window : PistonWindow,
    graph : Graph
}

impl MainWindow {
    // Constructor
    fn new() -> Self {
        MainWindow {
            window: WindowSettings::new(
                "Hello Piston!",
                [640, 480])
                .exit_on_esc(true)
                .build()
                .unwrap(),
            graph: Graph::new()
        }
    }

    fn init(&mut self) -> &mut Self {
        self.window.set_max_fps(60);
        self.window.set_title("Graph Example".to_owned());
        return self;
    }

    fn draw(&mut self) {
 
        self.graph.set_position(20.0, 10.0);

        while let Some(event) = self.window.next() {
            match event {
                Loop(Render(_)) => {
                    self.graph.draw(&mut self.window, &event)            
                }
                Loop(Update(args)) => {
                    self.graph.update(args.dt)
                }
                _ => {

                }
            }

        }
    }
}


fn main() {

    let mut window : MainWindow = MainWindow::new();
    window.init().draw();
}