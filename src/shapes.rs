
type Shape = (Vec<f32>, Vec<u32>);

pub fn getShape() -> Shape {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);
    if args.len() <= 1 {
        return getASingleTriangle()
    }

    let task = &args[1];

    return match task.as_str() {
        "1c" => getFiveTriangles(),
        _ => getASingleTriangle()
    }
}

// Task 1 c)
fn getFiveTriangles() -> Shape {
    (vec![], vec![])
}

fn getASingleTriangle() -> Shape {
    let vertices = vec![
        -0.6, -0.6, 0.0,    //v1
        0.6, -0.6, 0.0,     //v2
        0.0, 0.6, 0.0       //v3
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}
