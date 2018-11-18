extern crate piston_window;

use piston_window::*;

#[derive(Clone, Copy)]
pub struct Vector2f {
    pub x: f64,
    pub y: f64
}

trait Size {
    fn new(x: f64, y:f64) -> Self;
    fn resize(&mut self, x: f64, y:f64);
}

trait Position {
    fn new(x: f64, y:f64) -> Self;
    fn move_position(&mut self, x: f64, y:f64);
}

impl Size for Vector2f{

    fn new(x: f64, y:f64) -> Self {
        return Vector2f{
            x: x,
            y: y
        }
    }

    fn resize(&mut self, x: f64, y:f64) {
        self.x = x;
        self.y = y;
    }

}

impl Position for Vector2f{

    fn new(x: f64, y:f64) -> Self {
        return Vector2f{
            x: x,
            y: y
        }
    }

    fn move_position(&mut self, x: f64, y:f64) {
        self.x = x;
        self.y = y;
    }

}

pub struct Color {
    r: f32,
    b: f32,
    g: f32,
    a: f32
}

impl Color {
    pub fn new(color: [f32; 4]) -> Self {
        return Color {
            r: color[0],
            b: color[1],
            g: color[2],
            a: color[3],            
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.r = color[0];
        self.b = color[1];
        self.g = color[2];
        self.a = color[3];
    }

    pub fn get_color(&self) -> [f32; 4] {
        return [self.r, self.b, self.g, self.a];
    }
}

pub struct Graph {
    size : Vector2f,
    position: Vector2f,
    color: Color,
    totalTime: f64,
    switchTime: f64
}

impl Graph {

    pub fn new() -> Self {
        return Graph {
            size : Size::new(600.0, 200.0),
            position: Position::new(0.0, 0.0),
            color: Color::new([0.0, 0.0, 0.0, 1.0]),
            totalTime: 0.0,
            switchTime: 1.0
        }
    }
    
    #[allow(dead_code)]
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.size.x = width;
        self.size.y = height;
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color.set_color(color);
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn update(&mut self, dt: f64) {
        self.totalTime += dt;
        if self.totalTime >= self.switchTime {
            self.position.y += 10.0;
            self.totalTime -= self.switchTime;
        }
    }

    pub fn draw(&mut self, window:&mut  PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics| {
                clear([1.0; 4], graphics);
                rectangle(self.color.get_color(), // red
                        [self.position.x, self.position.y, self.size.x, self.size.y],
                        context.transform,
                        graphics);
            });
    }
}