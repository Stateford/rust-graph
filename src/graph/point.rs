use super::instance::Vector2f;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    position: i32
}

impl Point {

    fn new() -> Self {
        return Point{
            x: 0.0,
            y: 0.0,
            position: 0
        }
    }

    fn set_value(&mut self, value: f64, max_positions: i32, position: usize, size: Vector2f) {
        self.position = position as i32;
        let y: f64 = (size.y / 100.0) * value;
        let x: f64 = (size.x / max_positions as f64) * position as f64;
        self.y = y;
        self.x = x;
    }

    fn shift(&mut self, max_positions: i32, size: Vector2f) {
        self.position -= 1;
        let x: f64 = (size.x / max_positions as f64) * self.position as f64;
        self.x = x;
    }
    // #TODO: add rendering/drawing
}

pub struct GraphPoints {
    points: Vec<Point>,
    size: i32,
    max_size: i32,
    graph_size: Vector2f
}

impl GraphPoints {

    pub fn new(size: Vector2f) -> Self {
        return GraphPoints {
            points: Vec::new(),
            size: 0,
            max_size: 20,
            graph_size: size.clone()
        }
    }

    pub fn set_max_size(&mut self, size: i32) {
        self.max_size = size;
    }

    pub fn len(&self) -> i32 {
        return self.size.clone();
    }

    pub fn add_point(&mut self, value: f64) {
        
        if self.size >= self.max_size {
            self.points.drain(0..0);
        }

        let mut point = Point::new();
        point.set_value(
            value,
            self.max_size,
            self.points.len(),
            self.graph_size.clone());

        self.points.push(point);
        self.size += 1;
    }
}