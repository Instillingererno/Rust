fn main() {
    let mut test = make_3d_vector(5,5, 5);
    test[1][1][1] = 2.0;
}

fn make_2d_vector(x: usize, y: usize) -> Vec<Vec<f32>> {
    let mut temp: Vec<Vec<f32>> = Vec::with_capacity(x);
    for _ in 0..x {
        let mut new_vector: Vec<f32> = Vec::with_capacity(y);
        for _ in 0..y {
            new_vector.push(0.0);
        }
        temp.push(new_vector);
    }
    temp
}

fn make_3d_vector(x: usize, y: usize, z: usize) -> Vec<Vec<Vec<f32>>> {
    let mut temp: Vec<Vec<Vec<f32>>> = Vec::with_capacity(x);
    for _ in 0..x {
        temp.push(make_2d_vector(y,z));
    }
    temp
}