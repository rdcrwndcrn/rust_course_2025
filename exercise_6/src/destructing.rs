#![allow(dead_code)]

#[derive(Debug)]
struct DataContainer {
    coordinate: (i32, i32),
    precision: f64,
}

fn main() {
    let data_containers = [
        DataContainer {
            coordinate: (32, 128),
            precision: 0.005,
        },
        DataContainer {
            coordinate: (2, 2),
            precision: 0.005,
        },
        DataContainer {
            coordinate: (76, 37),
            precision: 0.25,
        },
        DataContainer {
            coordinate: (16, 24),
            precision: 1.0,
        },
        DataContainer {
            coordinate: (2, 2),
            precision: 0.05,
        },
        DataContainer {
            coordinate: (43, 24),
            precision: 0.5,
        },
    ];

    for container in data_containers.iter() {
        match container {
            // @TODO Add your pattern for destructuring
        }
    }
}
