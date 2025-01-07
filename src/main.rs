use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

#[derive(Parser)]
#[command(name = "fussy-food")]
#[command(about = "A CLI tool for helping us manage our toddler's meals.", long_about = "A CLI tool for helping us manage our family's meals. We want a place that can help us find recipes, come up with ideas for meals, and help us manage the meal plan for a toddler.", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum MealType {
    Breakfast = 1,
    Lunch = 2,
    Dinner = 3,
    Snack = 7,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage toddler meal planning
    Toddler {
        /// Type of meal to plan (breakfast=1, lunch=2, dinner=3, snack=7)
        #[arg(short, long, value_enum)]
        meal: MealType,
    },
}

fn get_random_fruit() -> &'static str {
    let fruits = ["apple", "banana", "orange", "strawberry", "blueberry"];
    fruits.choose(&mut rand::thread_rng()).unwrap()
}

fn breakfast_suggestion() -> &'static str {
    get_random_fruit()
}

fn lunch_suggestion() -> &'static str {
    get_random_fruit()
}

fn dinner_suggestion() -> &'static str {
    get_random_fruit()
}

fn snack_suggestion() -> &'static str {
    get_random_fruit()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Toddler { meal } => {
            let suggestion = match meal {
                MealType::Breakfast => breakfast_suggestion(),
                MealType::Lunch => lunch_suggestion(),
                MealType::Dinner => dinner_suggestion(),
                MealType::Snack => snack_suggestion(),
            };
            println!("Suggested fruit for {:?}: {}", meal, suggestion);
        }
    }
}
