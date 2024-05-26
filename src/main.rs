use knapsack::{AppArgs, Knapsack};
use tap::Tap;

fn main() -> anyhow::Result<()> {
    // Parsing user input
    let app_args: AppArgs = clap::Parser::parse();
    let parsed_objs = knapsack::app::parse_stdin()?;
    // The algorithm
    let sorted_objs = parsed_objs.tap_mut(|unsorted_objs| {
        unsorted_objs.sort_unstable_by_key(|obj| std::cmp::Reverse(obj.volume()))
    });
    let most_valuable_knapsack =
        Knapsack::filled_with_most_valuable(app_args.capacity(), sorted_objs.iter().copied());
    // Output for user
    dbg!(most_valuable_knapsack);
    Ok(())
}
