use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use sqlx::PgPool;
use std::error::Error;

#[derive(sqlx::FromRow, Debug, Clone)]
struct Food {
    id: i32,
    name: String,
    food_type: String,
    toddler_approved: bool,
    is_vegetarian: bool,
    is_vegan: bool,
    is_pescatarian: bool,
}

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(type_name = "common_allergy")]
#[sqlx(rename_all = "snake_case")]
pub enum CommonAllergy {
    Dairy,
    Eggs,
    Peanuts,
    TreeNuts,
    Soy,
    Wheat,
    Fish,
    Shellfish,
    Sesame,
}

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
        /// Dietary restriction to apply
        #[arg(short, long, value_enum, default_value = "none")]
        diet: DietaryRestriction,
    },
    /// Manage family meal planning
    Family {
        /// Type of meal to plan (breakfast=1, lunch=2, dinner=3, snack=7)
        #[arg(short, long, value_enum)]
        meal: MealType,
        /// Dietary restriction to apply
        #[arg(short, long, value_enum, default_value = "none")]
        diet: DietaryRestriction,
    },
}

async fn get_food_allergies(pool: &PgPool, food_id: i32) -> Result<Vec<CommonAllergy>, Box<dyn Error>> {
    let allergies = sqlx::query!(
        r#"SELECT allergy as "allergy: CommonAllergy" FROM food_allergies WHERE food_id = $1"#,
        food_id
    )
    .fetch_all(pool)
    .await?;
    
    Ok(allergies
        .into_iter()
        .filter_map(|r| r.allergy)
        .collect())
}

async fn get_random_fruit(
    pool: &PgPool,
    vegetarian: bool,
    vegan: bool,
    pescatarian: bool,
    toddler_only: bool,
) -> Result<Food, Box<dyn Error>> {
    let foods: Vec<Food> = sqlx::query_as!(
        Food,
        "SELECT * FROM foods 
         WHERE food_type = 'fruit' 
         AND ($1 = false OR is_vegetarian = true)
         AND ($2 = false OR is_vegan = true)
         AND ($3 = false OR is_pescatarian = true)
         AND ($4 = false OR toddler_approved = true)",
        vegetarian,
        vegan,
        pescatarian,
        toddler_only
    )
    .fetch_all(pool)
    .await?;

    foods
        .choose(&mut rand::thread_rng())
        .cloned()
        .ok_or_else(|| "No suitable fruits found".into())
}

async fn get_random_vegetable(
    pool: &PgPool,
    vegetarian: bool,
    vegan: bool,
    pescatarian: bool,
    toddler_only: bool,
) -> Result<Food, Box<dyn Error>> {
    let foods: Vec<Food> = sqlx::query_as!(
        Food,
        "SELECT * FROM foods 
         WHERE food_type = 'vegetable' 
         AND ($1 = false OR is_vegetarian = true)
         AND ($2 = false OR is_vegan = true)
         AND ($3 = false OR is_pescatarian = true)
         AND ($4 = false OR toddler_approved = true)",
        vegetarian,
        vegan,
        pescatarian,
        toddler_only
    )
    .fetch_all(pool)
    .await?;

    foods
        .choose(&mut rand::thread_rng())
        .cloned()
        .ok_or_else(|| "No suitable vegetables found".into())
}

async fn get_meal_suggestions(
    pool: &PgPool,
    vegetarian: bool,
    vegan: bool,
    pescatarian: bool,
    toddler_only: bool,
) -> Result<Vec<Food>, Box<dyn Error>> {
    let mut suggestions = Vec::new();
    
    // Always try to get a fruit
    if let Ok(fruit) = get_random_fruit(pool, vegetarian, vegan, pescatarian, toddler_only).await {
        suggestions.push(fruit);
    }
    
    // Always try to get a vegetable
    if let Ok(vegetable) = get_random_vegetable(pool, vegetarian, vegan, pescatarian, toddler_only).await {
        suggestions.push(vegetable);
    }
    
    Ok(suggestions)
}

async fn print_food_suggestion(pool: &PgPool, food: &Food) -> Result<(), Box<dyn Error>> {
    let allergies = get_food_allergies(pool, food.id).await?;
    println!("Suggested food: {}", food.name);
    println!("Type: {}", food.food_type);
    println!("Dietary info:");
    println!("  - Vegetarian: {}", if food.is_vegetarian { "Yes" } else { "No" });
    println!("  - Vegan: {}", if food.is_vegan { "Yes" } else { "No" });
    println!("  - Pescatarian: {}", if food.is_pescatarian { "Yes" } else { "No" });
    
    if !food.toddler_approved {
        println!("\n⚠️  Warning: This food is not toddler-approved and may need substitution!");
    }
    
    if !allergies.is_empty() {
        println!("\nContains allergens:");
        for allergy in allergies {
            println!("  - {:?}", allergy);
        }
    }
    Ok(())
}

async fn print_meal_suggestions(pool: &PgPool, foods: &[Food]) -> Result<(), Box<dyn Error>> {
    println!("\nMeal Suggestions:");
    println!("----------------");
    for food in foods {
        println!("\n🍽️  Option:");
        print_food_suggestion(pool, food).await?;
    }
    Ok(())
}

async fn breakfast_suggestion(pool: &PgPool, vegetarian: bool, vegan: bool, pescatarian: bool, toddler_only: bool) -> Result<(), Box<dyn Error>> {
    let suggestions = get_meal_suggestions(pool, vegetarian, vegan, pescatarian, toddler_only).await?;
    print_meal_suggestions(pool, &suggestions).await
}

async fn lunch_suggestion(pool: &PgPool, vegetarian: bool, vegan: bool, pescatarian: bool, toddler_only: bool) -> Result<(), Box<dyn Error>> {
    let suggestions = get_meal_suggestions(pool, vegetarian, vegan, pescatarian, toddler_only).await?;
    print_meal_suggestions(pool, &suggestions).await
}

async fn dinner_suggestion(pool: &PgPool, vegetarian: bool, vegan: bool, pescatarian: bool, toddler_only: bool) -> Result<(), Box<dyn Error>> {
    let suggestions = get_meal_suggestions(pool, vegetarian, vegan, pescatarian, toddler_only).await?;
    print_meal_suggestions(pool, &suggestions).await
}

async fn snack_suggestion(pool: &PgPool, vegetarian: bool, vegan: bool, pescatarian: bool, toddler_only: bool) -> Result<(), Box<dyn Error>> {
    // For snacks, just get a fruit
    let food = get_random_fruit(pool, vegetarian, vegan, pescatarian, toddler_only).await?;
    print_food_suggestion(pool, &food).await
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum DietaryRestriction {
    None,
    Vegetarian,
    Vegan,
    Pescatarian,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();
    
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?)
        .await?;

    match cli.command {
        Commands::Toddler { meal, diet } => {
            let (vegetarian, vegan, pescatarian) = match diet {
                DietaryRestriction::None => (false, false, false),
                DietaryRestriction::Vegetarian => (true, false, false),
                DietaryRestriction::Vegan => (true, true, false),
                DietaryRestriction::Pescatarian => (false, false, true),
            };
            match meal {
                MealType::Breakfast => breakfast_suggestion(&pool, vegetarian, vegan, pescatarian, true).await?,
                MealType::Lunch => lunch_suggestion(&pool, vegetarian, vegan, pescatarian, true).await?,
                MealType::Dinner => dinner_suggestion(&pool, vegetarian, vegan, pescatarian, true).await?,
                MealType::Snack => snack_suggestion(&pool, vegetarian, vegan, pescatarian, true).await?,
            }
        }
        Commands::Family { meal, diet } => {
            let (vegetarian, vegan, pescatarian) = match diet {
                DietaryRestriction::None => (false, false, false),
                DietaryRestriction::Vegetarian => (true, false, false),
                DietaryRestriction::Vegan => (true, true, false),
                DietaryRestriction::Pescatarian => (false, false, true),
            };
            match meal {
                MealType::Breakfast => breakfast_suggestion(&pool, vegetarian, vegan, pescatarian, false).await?,
                MealType::Lunch => lunch_suggestion(&pool, vegetarian, vegan, pescatarian, false).await?,
                MealType::Dinner => dinner_suggestion(&pool, vegetarian, vegan, pescatarian, false).await?,
                MealType::Snack => snack_suggestion(&pool, vegetarian, vegan, pescatarian, false).await?,
            }
        }
    }
    Ok(())
}
