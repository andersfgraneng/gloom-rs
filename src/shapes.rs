
type Shape = (Vec<f32>, Vec<u32>);

pub fn getShape() -> Shape {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        return getATestTriangle()
    }

    let task = &args[1];

    return match task.as_str() {
        "1c" => getFiveTriangles(),
        "2a" => getASingleTriangle(),
        _ => getATestTriangle()
    }
}

// Task 1 c)
fn getFiveTriangles() -> Shape {
    let vertices = vec![
        -0.9, -0.9, 0.0,
        -0.6, -0.6, 0.0,
        -0.9, -0.6, 0.0,

        -0.9, -0.5, 0.0,
        -0.6, -0.5, 0.0,
        -0.9, -0.2, 0.0,

        -0.5, -0.9, 0.0,
        -0.3, -0.3, 0.0,
        -0.4, 0.0, 0.0,

        0.3, 0.5, 0.0,
        0.0, -0.4, 0.0,
        0.6, -0.4, 0.0,

        -0.6, 0.0, 0.0,
        -0.3, 0.3, 0.0,
        -0.9, 0.9, 0.0
    ];

    let indices = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
        12, 13, 14
    ];

    (vertices, indices)
}

fn getASingleTriangle() -> Shape {
    let vertices = vec![
        0.6, -0.8, -1.2,
        0.0, 0.4, 0.0,
        -0.8, -0.2, 1.2
    ];

    let indices = vec![
        0, 1, 2,
    ];

    (vertices, indices)
}

fn getATestTriangle() -> Shape {
    let vertices = vec![
        -0.6, -0.6, 0.0,    //v1
        0.6, -0.6, 0.0,     //v2
        0.0, 0.6, 0.0       //v3
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}
