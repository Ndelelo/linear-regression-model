mod data;
mod model;

use data::generate_data;
use model::LinearRegression;
use burn::tensor::Tensor;
use burn::optim::{Optimizer, Sgd};
use textplots::{Chart, Plot, Shape};
use rgb::RGB8;

fn main() {
    let (x_data, y_data) = generate_data(100);
    let x_tensor = Tensor::from(x_data.to_vec());
    let y_tensor = Tensor::from(y_data.to_vec());

    let mut model = LinearRegression::new();
    let mut optimizer = SGD::new(0.01);

    for epoch in 0..1000 {
        let predictions = model.forward(&x_tensor);
        let loss = model.loss(&predictions, &y_tensor);

        optimizer.zero_grad();
        loss.backward();
        optimizer.step();

        if epoch % 100 == 0 {
            println!("Epoch {}: Loss = {}", epoch, loss.item());
        }
    }

    println!("Training complete. Weight: {}, Bias: {}", model.weight.item(), model.bias.item());

    // Plot the results
    let predicted: Vec<f32> = model.forward(&x_tensor).to_vec(); // Change to Vec<f32>
    let data: Vec<(f32, f32)> = x_data.iter().zip(predicted.iter()).map(|(&x, &y)| (x, y)).collect();
    Chart::new(180, 60, 0.0, 10.0)
        .lineplot(&Shape::Points(&data))
        .display();

    // Print a message in color using rgb
    let color = RGB8::new(255, 0, 0);
    println!("\x1b[38;2;{};{};{}mTraining complete!\x1b[0m", color.r, color.g, color.b);
}
