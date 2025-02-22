use burn::tensor::{Tensor, backend::Backend};
use burn::module::Module;
use burn_ndarray::NdArrayBackend;


pub struct LinearRegression<B: Backend> {
    pub(crate) weight: Tensor<B, 1>, // 1D tensor
    pub(crate) bias: Tensor<B, 1>,   // 1D tensor
}

impl<B: Backend> LinearRegression<B> {
    pub fn new() -> Self {
        Self {
            // Random initialization of weight and bias (1D tensor)
            weight: Tensor::randn([1], B::default()),
            bias: Tensor::randn([1], B::default()),
        }
    }

    pub fn forward(&self, x: Tensor<B, 1>) -> Tensor<B, 1> {
        x * self.weight.clone() + self.bias.clone()
    }

    /// Manually calculate Mean Squared Error (MSE) loss
    pub fn loss(&self, predictions: Tensor<B, 1>, targets: Tensor<B, 1>) -> Tensor<B, 1> {
        let diff = predictions - targets;
        diff.powf(2.0).mean()
    }
}

impl<B: Backend> Module for LinearRegression<B> {}




