mod abs;
mod avg;
pub mod command;
mod max;
mod min;
mod product;
mod reducers;
mod round;
mod sqrt;
mod sum;
mod utils;

pub use abs::SubCommand as MathAbs;
pub use avg::SubCommand as MathAvg;
pub use command::MathCommand as Math;
pub use max::SubCommand as MathMax;
pub use min::SubCommand as MathMin;
pub use product::SubCommand as MathProduct;
pub use round::SubCommand as MathRound;
pub use sqrt::SubCommand as MathSqrt;
pub use sum::SubCommand as MathSum;