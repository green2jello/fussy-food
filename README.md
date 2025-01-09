# Fussy Food CLI

A command-line tool for managing family meals with toddler-friendly options. This tool helps plan meals while considering dietary restrictions and toddler food preferences.

## Features

- **Meal Planning**
  - Separate toddler and family meal suggestions
  - Multiple food options per meal (fruit + vegetable)
  - Warnings for non-toddler-approved foods in family meals
  - Support for different meal types (breakfast, lunch, dinner, snacks)

- **Dietary Restrictions**
  - Vegetarian
  - Vegan
  - Pescatarian

- **Food Information**
  - Allergen warnings
  - Dietary compliance indicators
  - Toddler approval status

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- `libpq` (PostgreSQL C library)

## Setup

1. Install PostgreSQL and start the service:
```bash
brew install postgresql
brew services start postgresql
```

2. Create your user database:
```bash
createdb your_username
```

3. Set up the application database and schema:
```bash
psql -f schema.sql
```

4. Build the project:
```bash
cargo build
```

## Usage

### Toddler Meal Planning
```bash
# Get breakfast suggestion for toddler (vegetarian)
cargo run -- toddler -m breakfast -d vegetarian

# Get lunch suggestion for toddler (vegan)
cargo run -- toddler -m lunch -d vegan

# Get dinner suggestion for toddler (no restrictions)
cargo run -- toddler -m dinner -d none

# Get snack suggestion for toddler
cargo run -- toddler -m snack -d none
```

### Family Meal Planning
```bash
# Get breakfast suggestion for family (vegetarian)
cargo run -- family -m breakfast -d vegetarian

# Get dinner suggestion for family (pescatarian)
cargo run -- family -m dinner -d pescatarian
```

## Environment Variables

Create a `.env` file in the project root:
```
DATABASE_URL=postgres://localhost/fussy_food
```

## Future Enhancements

- CRUD operations for managing the food database
- Test coverage
- Additional meal types and food categories
- Meal planning and scheduling
- Recipe suggestions
- Nutritional information

## License

This project is licensed under the MIT License - see the LICENSE file for details.
