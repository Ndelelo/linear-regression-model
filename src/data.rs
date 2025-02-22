use rand::Rng;

pub fn generate_data(n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::rng(); // Create a random number generator
    let mut x_data = vec![0.0; n];
    let mut y_data = vec![0.0; n];

    for i in 0..n {
        let x: f32 = rng.random_range(0.0..10.0); // Generate x in the range [0.0, 10.0)
        let noise: f32 = rng.random_range(-1.0..1.0); // Generate noise in the range [-1.0, 1.0)
        let y = 2.0 * x + 1.0 + noise; // Generate a noisy y-value
        x_data[i] = x;
        y_data[i] = y;
    }

    (x_data, y_data)
}

