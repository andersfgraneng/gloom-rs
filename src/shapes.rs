
type Shape = (Vec<f32>, Vec<u32>);

pub fn getShape() -> Shape {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        return get_a_triangle()
    }

    let task = &args[1];

    return match task.as_str() {
        "1c" => get_1c_triangles(),
        "2a" => get_2a_triangle(),
        "2b" => get_2b_triangle(),
        _ => get_a_triangle()
    }
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
