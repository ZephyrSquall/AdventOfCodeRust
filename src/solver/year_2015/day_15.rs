use crate::solver::{Solution, Solver};
use std::cmp::max;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 15,
    title: "Science for Hungry People",
    part_solvers: &[solve_1, solve_2],
};

fn solve_1(input: &str) -> Solution {
    let ingredients = get_ingredients(input);
    let cookie = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    Solution::I32(find_highest_score(&ingredients, 100, false, 0, &cookie))
}

fn solve_2(input: &str) -> Solution {
    let ingredients = get_ingredients(input);
    let cookie = Ingredient {
        capacity: 0,
        durability: 0,
        flavor: 0,
        texture: 0,
        calories: 0,
    };

    Solution::I32(find_highest_score(&ingredients, 100, true, 0, &cookie))
}

// The ingredients' names are not important, only their stats are.
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn get_ingredients(input: &str) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();

    for line in input.lines() {
        // If the input string is split on commas, the number wanted is the final "word" of each of
        // the resulting strings. So after splitting by comma, split each string by spaces and parse
        // the last "word" into a number.
        let mut word_group_iter = line.split(',');

        let capacity = word_group_iter
            .next()
            .expect("Line should have first word group")
            .split(' ')
            .next_back()
            .expect("Word group should have last word")
            .parse()
            .expect("Capacity should be a number");
        let durability = word_group_iter
            .next()
            .expect("Line should have second word group")
            .split(' ')
            .next_back()
            .expect("Word group should have last word")
            .parse()
            .expect("Durability should be a number");
        let flavor = word_group_iter
            .next()
            .expect("Line should have third word group")
            .split(' ')
            .next_back()
            .expect("Word group should have last word")
            .parse()
            .expect("Flavor should be a number");
        let texture = word_group_iter
            .next()
            .expect("Line should have fourth word group")
            .split(' ')
            .next_back()
            .expect("Word group should have last word")
            .parse()
            .expect("Texture should be a number");
        let calories = word_group_iter
            .next()
            .expect("Line should have fifth word group")
            .split(' ')
            .next_back()
            .expect("Word group should have last word")
            .parse()
            .expect("Calories should be a number");

        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }

    ingredients
}

// Checks every possible mixture of the provided ingredients that adds up to exactly the provided
// number of teaspoons of total ingredients (i.e. with 4 ingredients and 100 teaspoons, it will
// check every possible way of dividing the 4 ingredients so 100 teaspoons are used in total), and
// returns the mixture with the highest score. This function uses recursion; depth should be 0 and
// cookie should be an ingredient with 0 in every category when initially calling it.
fn find_highest_score(
    ingredients: &[Ingredient],
    mut teaspoons: i32,
    has_calorie_limit: bool,
    // depth indicates which ingredient (by index) is currently being added to the cookie. Each
    // level of recursion moves on to the next ingredient.
    depth: usize,
    // cookie tracks the total stats of the ingredients added so far. Each level of recursion adds
    // the stats of that level's ingredient to the cookie's stats.
    cookie: &Ingredient,
) -> i32 {
    // Terminate the recursion and calculate the final score when depth == ingredients.len() - 1,
    // indicating that the final ingredient has been reached. The only possible amount of teaspoons
    // to use for the final ingredient is all of the unused remaining teaspoons (otherwise not all
    // cookie mixtures will use the same number of teaspoons of ingredients overall) so no further
    // looping is necessary.
    if depth == ingredients.len() - 1 {
        // First add the final ingredient to the cookie.
        let this_ingredient = &ingredients[depth];
        let final_cookie = Ingredient {
            capacity: cookie.capacity + (teaspoons * this_ingredient.capacity),
            durability: cookie.durability + (teaspoons * this_ingredient.durability),
            flavor: cookie.flavor + (teaspoons * this_ingredient.flavor),
            texture: cookie.texture + (teaspoons * this_ingredient.texture),
            calories: cookie.calories + (teaspoons * this_ingredient.calories),
        };

        if has_calorie_limit && final_cookie.calories != 500 {
            // If the mixture doesn't have the right number of calories, Remove this mixture from
            // the results by returning 0, ensuring it can't be the highest-scoring mixture.
            0
        } else {
            // Calculate and return the score. Zero out any ingredient category that is negative.
            max(final_cookie.capacity, 0)
                * max(final_cookie.durability, 0)
                * max(final_cookie.flavor, 0)
                * max(final_cookie.texture, 0)
        }
    } else {
        let mut highest_score = 0;
        let this_ingredient = &ingredients[depth];
        let starting_teaspoons = teaspoons;

        while teaspoons >= 0 {
            // Calculate the new stats of the cookie once the current teaspoon amount of the current
            // ingredient is added to the cookie. These are not yet the final cookie stats, but
            // these stats will be added to further in later recursive calls to this function.
            let new_cookie = Ingredient {
                capacity: cookie.capacity + (teaspoons * this_ingredient.capacity),
                durability: cookie.durability + (teaspoons * this_ingredient.durability),
                flavor: cookie.flavor + (teaspoons * this_ingredient.flavor),
                texture: cookie.texture + (teaspoons * this_ingredient.texture),
                calories: cookie.calories + (teaspoons * this_ingredient.calories),
            };
            let score = find_highest_score(
                ingredients,
                // Ensure the total teaspoons remains the same by recursively calling this function
                // with the difference between the starting number of teaspoons and the teaspoons
                // used so far.
                starting_teaspoons - teaspoons,
                has_calorie_limit,
                // Increase the depth by one so the recursively-called function moves onto the next
                // ingredient in the ingredients vector.
                depth + 1,
                &new_cookie,
            );
            if score > highest_score {
                highest_score = score;
            }

            // Because this function was called recursively, by the time execution reaches the end
            // of this loop, every possible mixture of ingredients that come after this one with the
            // teaspoons that were remaining have been checked. So reduce the teaspoons and repeat
            // everything again until every possible value of teaspoons for the current ingredient
            // are checked.
            teaspoons -= 1;
        }

        highest_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_1(
                "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            Solution::U32(62_842_880)
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_2(
                "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            Solution::U32(57_600_000)
        );
    }
}
