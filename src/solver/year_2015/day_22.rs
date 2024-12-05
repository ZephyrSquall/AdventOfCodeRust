use crate::solver::{Solution, Solver};
use std::cmp::{max, min};
use std::mem::discriminant;

// Suppress warnings while this solver is excluded.
#[allow(dead_code)]
pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 22,
    title: "Wizard Simulator 20XX",
    part_solvers: &[solve_1],
};

// I'm mostly confident the logic of this solver is sound. However, the solver did not terminate
// after two hours of running, so regardless of whether this produces the correct answer, it is
// clearly not the intended way to solve this problem. I currently have no ideas for how I can
// improve the speed of this solver or solve it with a different algorithm, so I will come back to
// this problem later.
fn solve_1(input: &str) -> Solution {
    // A recursive function that executes all events between the player casting a chosen spell and
    // the player choosing which spell to cast on their next turn. This doesn't entirely line up
    // with the turn start since it begins partway into the player's turn, but no part of a turn is
    // skipped if this function is called repeatedly. It returns the amount of mana spent if the
    // player wins the fight, of i32::MAX if the player lost the fight (as i32::MAX cannot possibly
    // be a new minimum amount of spent mana).
    fn turn_cycle(
        spell: &Spell,
        player: &mut Player,
        boss: &mut Boss,
        effects: &mut Vec<Effect>,
        mut mana_spent: i32,
        mana_spent_upper_limit: i32,
    ) -> i32 {
        // Add the mana cost of the chosen spell to the total mana spent. If this goes above the
        // upper limit, then a path to victory that spends less mana than this has already been
        // found. Continuing to explore this branch is pointless because any victories found beyond
        // this point will spend more mana than this, so immediately return as if the fight was
        // lost.
        mana_spent += spell.mana_cost;
        if mana_spent >= mana_spent_upper_limit {
            return i32::MAX;
        }

        // Cast the chosen spell. Drain the player's mana, damage the boss, heal the player, and set
        // up any effects.
        player.mana -= spell.mana_cost;
        boss.hit_points -= spell.damage;
        player.hit_points += spell.healing;
        if let Some(effect) = spell.effect.clone() {
            effects.push(effect);
        }

        // Wait to check if the boss was defeated until after applying poison damage.

        // The boss's turn starts. First apply all effects. Vec's retain_mut method is perfect for
        // this, as it will call the provided predicate on each of its elements and allow the
        // predicate to mutate its element to reduce the turn count. This this line calls the apply
        // method of all effects, and then removes all effects whose apply method returned false
        // (which the apply method does when the effect runs out of turns and expires).
        effects.retain_mut(|effect| effect.apply(player, boss));
        // The boss could have been defeated by poison damage, so check if the player just won the
        // fight.
        if boss.hit_points <= 0 {
            return mana_spent;
        }

        // Effects have been applied, so now the boss attacks. Determine damage, reduce the player's
        // HP, then check if the player just lost the fight.
        let boss_damage_dealt = max(1, boss.damage - player.armor);
        player.hit_points -= boss_damage_dealt;
        if player.hit_points <= 0 {
            return i32::MAX;
        }

        // The player's turn starts. First apply all effects.
        effects.retain_mut(|effect| effect.apply(player, boss));
        if boss.hit_points <= 0 {
            return mana_spent;
        }

        // Iterate over all five spells the player has. If the player is able to cast it (the player
        // has sufficient mana and the spell doesn't set up an effect that's already in effect),
        // recursively call this function with that spell to explore all possible actions the player
        // can take.
        let mut minimum_victory_mana_spent = i32::MAX;
        for spell in &SPELLS {
            if player.mana >= spell.mana_cost
                && spell
                    .effect
                    .as_ref()
                    .is_none_or(|effect| !effects.contains(effect))
            {
                minimum_victory_mana_spent = min(
                    minimum_victory_mana_spent,
                    turn_cycle(
                        spell,
                        // The player's stats, boss's stats, and effects list must be cloned, so
                        // this branch doesn't affect other branches from this point.
                        &mut player.clone(),
                        &mut boss.clone(),
                        &mut effects.clone(),
                        mana_spent,
                        minimum_victory_mana_spent,
                    ),
                );
            }
        }

        // After the above loop has run, every possible fight that can occur from this moment
        // onwards has been explored, and the minimum amount of mana spent among all of those fights
        // has been obtained. Return this value so every branching point before this moment gets the
        // minimum mana spent.

        // Note that the player immediately loses if they are unable to cast any spell. However,
        // there is no need to explicitly test this condition. Should this occur, the above loop
        // will never call turn_cycle, which means minimum_victory_mana_spent will never be updated
        // to a value other than i32::MAX. Returning an i32::MAX is already how this function
        // indicates the player lost the fight, as i32::MAX can never update a minimum value.

        minimum_victory_mana_spent
    }

    let player = Player {
        hit_points: 50,
        mana: 500,
        armor: 0,
    };
    let boss = get_boss(input);
    let mut minimum_victory_mana_spent = i32::MAX;

    // For the first spell the player casts, there is no need to check if the player is able to cast
    // it, as on the first turn the player has sufficient mana to cast any spell and no effects have
    // been set up. It is also not necessary to apply effects at the start of the player's first
    // turn, as no effects have been set up. Thus, turn_cycle can be called immediately for all five
    // spells.
    for spell in &SPELLS {
        minimum_victory_mana_spent = min(
            minimum_victory_mana_spent,
            turn_cycle(
                spell,
                &mut player.clone(),
                &mut boss.clone(),
                &mut Vec::with_capacity(3),
                0,
                minimum_victory_mana_spent,
            ),
        );
    }

    Solution::I32(minimum_victory_mana_spent)
}

#[derive(Clone)]
struct Player {
    hit_points: i32,
    mana: i32,
    armor: i32,
}

#[derive(Clone)]
struct Boss {
    hit_points: i32,
    damage: i32,
}

struct Spell {
    mana_cost: i32,
    damage: i32,
    healing: i32,
    effect: Option<Effect>,
}

#[derive(Clone)]
enum Effect {
    Shield(u32),
    Poison(u32),
    Recharge(u32),
}
// An Effect should test as equal to another Effect if it is the same variant, even if it has a
// different number of turns remaining. Deriving the PartialEq trait will cause an Effect to only
// test as equal to another Effect if they both have the same variant and the same number of turns
// remaining, so PartialEq must be implemented manually to get the desired behaviour.
impl PartialEq for Effect {
    fn eq(&self, other: &Effect) -> bool {
        discriminant(self) == discriminant(other)
    }
}
// Applies the effect (raises the player's armor if it is Shield and the player just cast it, lowers
// the player's armor if it is Shield and it is about to expire, damages the boss if it is Poison,
// and replenishes the player's mana if it is Recharge). Returns a bool indicating whether the
// effect should remain (false if turns_remaining reached 0, otherwise true). This should be called
// in a retain_mut method of a Vec<Effect> to apply all currently-active effects at once and remove
// any which expired.
impl Effect {
    fn apply(&mut self, player: &mut Player, boss: &mut Boss) -> bool {
        match self {
            Effect::Shield(turns_remaining) => {
                if *turns_remaining == 6 {
                    player.armor = 7;
                } else if *turns_remaining == 1 {
                    player.armor = 0;
                }

                *turns_remaining -= 1;
                *turns_remaining != 0
            }
            Effect::Poison(turns_remaining) => {
                boss.hit_points -= 3;

                *turns_remaining -= 1;
                *turns_remaining != 0
            }
            Effect::Recharge(turns_remaining) => {
                player.mana += 101;

                *turns_remaining -= 1;
                *turns_remaining != 0
            }
        }
    }
}

const SPELLS: [Spell; 5] = [
    // Magic Missile
    Spell {
        mana_cost: 53,
        damage: 4,
        healing: 0,
        effect: None,
    },
    // Drain
    Spell {
        mana_cost: 73,
        damage: 2,
        healing: 2,
        effect: None,
    },
    // Shield
    Spell {
        mana_cost: 113,
        damage: 0,
        healing: 0,
        effect: Some(Effect::Shield(6)),
    },
    // Poison
    Spell {
        mana_cost: 173,
        damage: 0,
        healing: 0,
        effect: Some(Effect::Poison(6)),
    },
    // Recharge
    Spell {
        mana_cost: 229,
        damage: 0,
        healing: 0,
        effect: Some(Effect::Recharge(5)),
    },
];

fn get_boss(input: &str) -> Boss {
    let mut boss = Boss {
        hit_points: 0,
        damage: 0,
    };
    for line in input.lines() {
        let mut colon_iter = line.split(": ");
        let stat_name = colon_iter.next().expect("Line should have a named stat");
        let stat_value = colon_iter
            .next()
            .expect("Line should have a stat value")
            .parse()
            .expect("Stat value should be a number");
        match stat_name {
            "Hit Points" => boss.hit_points = stat_value,
            "Damage" => boss.damage = stat_value,
            _ => panic!("Stat should be one of \"Hit Points\" or \"Damage\""),
        }
    }

    boss
}

// The puzzle description provides no examples for this puzzle.
