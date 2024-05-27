use knapsack::{AppArgs, FillingProgress, Knapsack};
use num_bigint::BigUint;
use num_traits::One;
use tap::Tap;

fn main() -> anyhow::Result<()> {
    // Parsing user input
    let app_args: AppArgs = clap::Parser::parse();
    let parsed_objs = knapsack::app::parse_stdin()?;
    // The algorithm
    let sorted_objs = parsed_objs.tap_mut(|unsorted_objs| {
        unsorted_objs.sort_unstable_by_key(|obj| std::cmp::Reverse(obj.volume()))
    });
    let mut filling_progress = FillingProgress::new(BigUint::one() << sorted_objs.len());
    let most_valuable = Knapsack::filled_with_most_valuable(
        app_args.capacity(),
        sorted_objs.iter().copied(),
        #[cfg(feature = "progress_bar")]
        app_args.progress_bar().then(|| &mut filling_progress),
    );
    // Output for user
    println!("{}", most_valuable);
    Ok(())
}
