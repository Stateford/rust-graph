extern crate graph;

use graph::graph::point::*;
use graph::graph::instance::*;

#[test]
fn test_add_point() {
    let size : Vector2f = Size::new(200.0, 200.0);
    let mut graph_points : GraphPoints = GraphPoints::new(size);

    graph_points.set_max_size(20);

    for i in 0..100 {
        graph_points.add_point(i as f64);
    }

    // assert_eq!(graph_points.len(), 20);
}
