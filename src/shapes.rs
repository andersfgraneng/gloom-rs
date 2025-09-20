
type ShapeWithColors = (Vec<f32>, Vec<u32>, Vec<f32>);
type Shape = (Vec<f32>, Vec<u32>);

pub fn get_shape() -> ShapeWithColors {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        return get_a2_1b_triangle()
    }

    let task = &args[1];

    return match task.as_str() {
//        "a1_1c" => get_1c_triangles(),
//        "a1_2a" => get_2a_triangle(),
//        "a1_2b" => get_2b_triangle(),
        "a2_1b" => get_a2_1b_triangle(),
        "a2_2a" => get_a2_2a_triangle(),
        _ => get_a2_1b_triangle()
    }
}

fn get_a2_2a_triangle() -> ShapeWithColors {
    let vertices = vec![

        // Color 2
        -0.9, -0.9, 0.1,
        -0.1, -0.5, 0.1,
        -0.9, -0.2, 0.1,

        // Color 1
        -0.9, -0.9, 0.0,
        -0.1, -0.6, 0.0,
        -0.9, -0.6, 0.0,

        // Color 3
        -0.5, -0.9, -0.2,
        -0.3, -0.3, -0.2,
        -0.4, 0.0, -0.2,
    ];

    let indices = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 0.7,
        1.0, 0.0, 0.0, 0.7,
        1.0, 0.0, 0.0, 0.7,

        0.0, 1.0, 0.0, 0.3,
        0.0, 1.0, 0.0, 0.3,
        0.0, 1.0, 0.0, 0.3,

        0.0, 0.0, 1.0, 0.5,
        0.0, 0.0, 1.0, 0.5,
        0.0, 0.0, 1.0, 0.5,
    ];

    (vertices, indices, colors)
}

fn get_a2_1b_triangle() -> ShapeWithColors {
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
    ];

    let indices = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0,

        1.0, 1.0, 0.0, 1.0,
        0.0, 1.0, 1.0, 1.0,
        0.0, 1.0, 1.0, 1.0,

        0.8, 0.5, 0.4, 1.0,
        0.5, 0.2, 0.9, 1.0,
        0.1, 0.6, 0.75, 1.0,
    ];

    (vertices, indices, colors)
}

// Task 1 c)
fn get_1c_triangles() -> Shape {
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

fn get_2a_triangle() -> Shape {
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

fn get_2b_triangle() -> Shape {
    let vertices = vec![
        -0.6, -0.6, 0.0,    
        0.6, -0.6, 0.0,     
        0.0, 0.6, 0.0       
    ];

    let indices = vec![0, 2, 1];

    return (vertices, indices)
}

fn get_a_triangle() -> Shape {
    let vertices = vec![
        -0.2, -0.6, 0.0,    //v1
        0.6, -0.6, 0.0,     //v2
        0.0, 0.6, 0.0       //v3
    ];

    let indices = vec![0, 1, 2];

    (vertices, indices)
}
