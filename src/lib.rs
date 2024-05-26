pub use knapsack::*;
pub mod knapsack;

pub use obj::Object;
pub mod obj;

pub use args::AppArgs;
pub mod args {
    use ordered_float::NotNan;

    #[derive(clap::Parser)]
    #[command(version)]
    pub struct AppArgs {
        capacity: NotNan<f64>,
    }

    impl AppArgs {
        pub fn capacity(&self) -> NotNan<f64> {
            self.capacity
        }
    }
}

pub mod app;
