use crate::solver::{Solution, Solver};
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::mem::discriminant;

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 22,
    title: "Wizard Simulator 20XX",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    // Since the player has to spend mana every turn, the fight which spends the minimum amount of
    // mana and still wins is overwhelmingly likely to occur in a low number of turns. The player
    // also has the means to stall out the fight for an extremely high number of turns by raising
    // their armor with Shield and healing the damage they take with Drain (from my testing, the
    // player could sometimes take more than 35 turns). This leads to the tree of all possible
    // fights being too large to search exhaustively.
    //
    // This makes this problem especially well-suited for a breadth-first search. This finds the
    // victories that take the fewest number of turns quickly. As the amount of mana spent can only
    // increase, any branching path of the fight that is found to spend more mana than the minimum
    // winning amount can immediately be terminated. This prunes huge amounts of fights that must be
    // explored from the tree of all possible fights, as most branches consist of stalled-out
    // fights.

    // This represents all the information that is required to be stored in the queue for the
    // breadth-first search. Note that mana_spent_upper_limit is not included since this value can
    // change between adding a turn cycle to the queue and executing that turn cycle.
    struct TurnCycleParameters<'a> {
        spell: &'a Spell,
        player: Player,
        boss: Boss,
        effects: Vec<Effect>,
        mana_spent: i32,
    }

    // This function executes all events between the player casting a chosen spell and the player
    // choosing which spell to cast on their next turn. This doesn't perfectly line up with the turn
    // start, since it begins partway into the player's turn. But this doesn't matter because when
    // this function is called repeatedly, turns still happen in their entirety. If the player wins
    // the fight during this turn cycle, this function returns the total amount of mana spent during
    // the fight. Otherwise, this function returns i32::MAX to ensure the current record for minimum
    // amount of mana spent to win the fight isn't affected. If the player neither won nor lost and
    // is about to choose their next spell, this function will add a set of turn cycle parameters
    // representing all possible turn cycles that can occur from this point to the turn cycle queue
    // (one for each spell the player can cast) before it returns i32::MAX.
    fn turn_cycle(
        mut params: TurnCycleParameters,
        mana_spent_upper_limit: i32,
        queue: &mut VecDeque<TurnCycleParameters>,
    ) -> i32 {
        // Add the mana cost of the chosen spell to the total mana spent. If this goes above the
        // upper limit, then a path to victory that spends less mana than this has already been
        // found. Continuing to explore this branch is pointless because any victories found beyond
        // this point will spend more mana than this, so immediately return as if the fight was
        // lost.
        params.mana_spent += params.spell.mana_cost;
        if params.mana_spent >= mana_spent_upper_limit {
            return i32::MAX;
        }

        // Cast the chosen spell. Drain the player's mana, damage the boss, heal the player, and set
        // up any effects.
        params.player.mana -= params.spell.mana_cost;
        params.boss.hit_points -= params.spell.damage;
        params.player.hit_points += params.spell.healing;
        if let Some(effect) = params.spell.effect.clone() {
            params.effects.push(effect);
        }

        // Wait to check if the boss was defeated until after applying poison damage.

        // The boss's turn starts now. First apply all effects. Vec's retain_mut method is perfect
        // for this, as it will call the provided predicate on each of its elements and allow the
        // predicate to mutate itself to reduce the turn count. Hence this line calls the apply
        // method of all effects, and then removes all effects whose apply method returned false
        // (which the apply method does when the effect runs out of turns and expires).
        params
            .effects
            .retain_mut(|effect| effect.apply(&mut params.player, &mut params.boss));
        // The boss could have been defeated by poison damage, or from damage the player dealt
        // earlier, so check if the player just won the fight.
        if params.boss.hit_points <= 0 {
            return params.mana_spent;
        }

        // Effects have been applied, so now the boss attacks. Determine damage, reduce the player's
        // HP, then check if the player just lost the fight.
        let boss_damage_dealt = max(1, params.boss.damage - params.player.armor);
        params.player.hit_points -= boss_damage_dealt;
        if params.player.hit_points <= 0 {
            return i32::MAX;
        }

        // The player's turn starts now. First apply all effects.
        params
            .effects
            .retain_mut(|effect| effect.apply(&mut params.player, &mut params.boss));
        if params.boss.hit_points <= 0 {
            return params.mana_spent;
        }

        // Iterate over all five spells the player has. If the player is able to cast it (the player
        // has sufficient mana and the spell doesn't set up an effect that's already in effect),
        // add the following turn where the player casts that spell to the turn cycle queue.
        for spell in &SPELLS {
            if params.player.mana >= spell.mana_cost
                && spell
                    .effect
                    .as_ref()
                    .is_none_or(|effect| !params.effects.contains(effect))
            {
                queue.push_back(TurnCycleParameters {
                    spell,
                    player: params.player.clone(),
                    boss: params.boss.clone(),
                    effects: params.effects.clone(),
                    mana_spent: params.mana_spent,
                });
            }
        }

        // Finally return i32::MAX since the player hasn't won the fight yet so the minimum winning
        // mana cost shouldn't be updated. Note that the player immediately loses if they are unable
        // to cast any spell, however there is no need to explicitly test this condition. Should
        // this occur, the above loop won't add any turns to the turn cycle queue, effectively
        // causing the branch to die here.
        i32::MAX
    }

    // Set up the initial stats of the player and boss, and initialize the mana spent record and the
    // turn cycle queue.
    let player = Player {
        hit_points: 50,
        mana: 500,
        armor: 0,
    };
    let boss = get_boss(input);
    let mut minimum_victory_mana_spent = i32::MAX;
    let mut turn_cycle_queue = VecDeque::new();

    // For the first spell the player casts, there is no need to check if the player is able to cast
    // it, as on the first turn the player has sufficient mana to cast any spell and no effects have
    // been set up. It is also not necessary to apply effects at the start of the player's first
    // turn, as no effects have been set up. Thus, turn_cycle can be called immediately for all five
    // spells.
    for spell in &SPELLS {
        turn_cycle_queue.push_back(TurnCycleParameters {
            spell,
            player: player.clone(),
            boss: boss.clone(),
            effects: Vec::with_capacity(3),
            mana_spent: 0,
        });
    }

    // Keep executing turn cycles until the queue runs out.
    while let Some(params) = turn_cycle_queue.pop_front() {
        // turn_cycle only returns a value other than i32::MAX if the player just won the fight, so
        // minimum_victory_mana_spent can only be changed if the player won.
        minimum_victory_mana_spent = min(
            minimum_victory_mana_spent,
            turn_cycle(params, minimum_victory_mana_spent, &mut turn_cycle_queue),
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
