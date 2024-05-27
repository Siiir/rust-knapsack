use ordered_float::NotNan;

#[derive(clap::Parser)]
#[command(version)]
pub struct AppArgs {
    capacity: NotNan<f64>,
    #[cfg(feature = "progress_bar")]
    #[arg(short, long, default_value_t = false)]
    progress_bar: bool,
}

impl AppArgs {
    pub fn capacity(&self) -> NotNan<f64> {
        self.capacity
    }
    #[cfg(feature = "progress_bar")]
    pub fn progress_bar(&self) -> bool {
        self.progress_bar
    }
}
