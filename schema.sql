CREATE DATABASE fussy_food;

\c fussy_food

-- Create enum for common allergies
CREATE TYPE common_allergy AS ENUM (
    'dairy',
    'eggs',
    'peanuts',
    'tree_nuts',
    'soy',
    'wheat',
    'fish',
    'shellfish',
    'sesame'
);

-- Create foods_allergies junction table
CREATE TABLE food_allergies (
    id SERIAL PRIMARY KEY,
    food_id INTEGER,
    allergy common_allergy,
    UNIQUE(food_id, allergy)
);

CREATE TABLE foods (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    food_type VARCHAR(50) NOT NULL,
    toddler_approved BOOLEAN NOT NULL DEFAULT false,
    is_vegetarian BOOLEAN NOT NULL DEFAULT false,
    is_vegan BOOLEAN NOT NULL DEFAULT false,
    is_pescatarian BOOLEAN NOT NULL DEFAULT false
);

-- Insert some sample data
INSERT INTO foods (name, food_type, toddler_approved, is_vegetarian, is_vegan, is_pescatarian) VALUES
    ('apple', 'fruit', true, true, true, true),
    ('banana', 'fruit', true, true, true, true),
    ('orange', 'fruit', true, true, true, true),
    ('strawberry', 'fruit', true, true, true, true),
    ('blueberry', 'fruit', true, true, true, true),
    ('broccoli', 'vegetable', false, true, true, true),
    ('carrot', 'vegetable', true, true, true, true),
    ('chicken nuggets', 'protein', true, false, false, false),
    ('pasta', 'grain', true, true, true, true),
    ('yogurt', 'dairy', true, true, false, true),
    ('salmon', 'protein', false, false, false, true),
    ('tofu', 'protein', false, true, true, true);

-- Add some sample allergen data
INSERT INTO food_allergies (food_id, allergy) VALUES
    (9, 'wheat'),     -- pasta contains wheat
    (10, 'dairy'),    -- yogurt contains dairy
    (8, 'wheat'),     -- chicken nuggets contain wheat
    (12, 'soy');      -- tofu contains soy
