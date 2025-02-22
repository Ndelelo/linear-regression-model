use burn::tensor::Tensor;
use burn::module::Module;

pub struct LinearRegression {
    pub(crate) weight: Tensor,
    pub(crate) bias: Tensor,
}

impl LinearRegression {
    pub fn new() -> Self {
        Self {
            weight: Tensor::randn(&[1], ()),
            bias: Tensor::randn(&[1], ()),
        }
    }

    pub fn forward(&self, x: &Tensor) -> Tensor { x * &self.weight + &self.bias }

    /// Manually calculate Mean Squared Error (MSE) loss
    pub fn loss(&self, predictions: &Tensor, targets: &Tensor) -> Tensor {
        let diff = predictions - targets;
        diff.powf(2.0).mean()
    }
}

impl Module for LinearRegression {}



