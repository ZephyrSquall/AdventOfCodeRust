use crate::solver::{Solution, Solver};
use std::cmp::{max, min};

pub const SOLVER: Solver = Solver {
    year: 2015,
    day: 21,
    title: "RPG Simulator 20XX",
    part_solvers: &[solve_1],
};

fn solve_1(input: &str) -> Solution {
    struct Fighter {
        hit_points: i32,
        damage: i32,
        armor: i32,
    }
    struct Equipment {
        cost: u32,
        damage: i32,
        armor: i32,
    }

    const WEAPONS: [Equipment; 5] = [
        // Dagger
        Equipment {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        // Shortsword
        Equipment {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        // Warhammer
        Equipment {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        // Longsword
        Equipment {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        // Greataxe
        Equipment {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    const ARMORS: [Equipment; 6] = [
        // Buy nothing
        Equipment {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        // Leather
        Equipment {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        // Chainmail
        Equipment {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        // Splintmail
        Equipment {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        // Bandedmail
        Equipment {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        // Platemail
        Equipment {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    const RINGS: [Equipment; 7] = [
        // Buy nothing
        Equipment {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        // Damage +1
        Equipment {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        // Damage +2
        Equipment {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        // Damage +3
        Equipment {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        // Defense +1
        Equipment {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        // Defense +2
        Equipment {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        // Defense +3
        Equipment {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    // The following four functions recursively check all possible equipment purchases to find the
    // cheapest set of equipment that still allows the player to win.
    fn purchase_weapon(purchased_equipment: &mut Vec<&Equipment>, boss: &Fighter) -> u32 {
        let mut minimum_victory_cost = u32::MAX;

        for weapon in &WEAPONS {
            purchased_equipment.push(weapon);
            minimum_victory_cost = min(
                minimum_victory_cost,
                purchase_armor(purchased_equipment, boss),
            );
            purchased_equipment.pop();
        }

        minimum_victory_cost
    }

    fn purchase_armor(purchased_equipment: &mut Vec<&Equipment>, boss: &Fighter) -> u32 {
        let mut minimum_victory_cost = u32::MAX;

        for armor in &ARMORS {
            purchased_equipment.push(armor);
            minimum_victory_cost = min(
                minimum_victory_cost,
                purchase_first_ring(purchased_equipment, boss),
            );
            purchased_equipment.pop();
        }

        minimum_victory_cost
    }

    fn purchase_first_ring(purchased_equipment: &mut Vec<&Equipment>, boss: &Fighter) -> u32 {
        let mut minimum_victory_cost = u32::MAX;

        // Track the index of the purchased ring. The second ring will always use a higher index to
        // ensure the same ring isn't purchased twice and each combination of two rings is only
        // checked once.
        for (index, ring) in RINGS.iter().enumerate() {
            purchased_equipment.push(ring);
            minimum_victory_cost = min(
                minimum_victory_cost,
                purchase_second_ring(purchased_equipment, index, boss),
            );
            purchased_equipment.pop();
        }

        // It is also possible to purchase no rings whatsoever, which is skipped in the above loop
        // as buying no ring twice is considered a duplicate purchase, so this must be checked
        // separately. Purchasing no rings has no effect on player stats, so the purchased_equipment
        // vector can simply be used as-is.
        minimum_victory_cost = min(minimum_victory_cost, fight(purchased_equipment, boss));

        minimum_victory_cost
    }

    fn purchase_second_ring(
        purchased_equipment: &mut Vec<&Equipment>,
        first_ring_index: usize,
        boss: &Fighter,
    ) -> u32 {
        let mut minimum_victory_cost = u32::MAX;

        for ring in RINGS.iter().skip(first_ring_index + 1) {
            purchased_equipment.push(ring);
            minimum_victory_cost = min(minimum_victory_cost, fight(purchased_equipment, boss));
            purchased_equipment.pop();
        }

        minimum_victory_cost
    }

    // Checks if the player wins the fight with their current equipment. If the player wins, returns
    // the total cost of the player's equipment. If the player loses, returns u32::MAX (as the
    // maximum possible u32 value can never be a new minimum value)
    fn fight(purchased_equipment: &Vec<&Equipment>, boss: &Fighter) -> u32 {
        let mut equipment_cost = 0;
        let mut player = Fighter {
            hit_points: 100,
            damage: 0,
            armor: 0,
        };

        for equipment in purchased_equipment {
            equipment_cost += equipment.cost;
            player.damage += equipment.damage;
            player.armor += equipment.armor;
        }

        let mut player_current_hit_points = player.hit_points;
        let mut boss_current_hit_points = boss.hit_points;

        let player_damage_dealt = max(1, player.damage - boss.armor);
        let boss_damage_dealt = max(1, boss.damage - player.armor);

        loop {
            boss_current_hit_points -= player_damage_dealt;
            if boss_current_hit_points <= 0 {
                return equipment_cost;
            }
            player_current_hit_points -= boss_damage_dealt;
            if player_current_hit_points <= 0 {
                return u32::MAX;
            }
        }
    }

    // Get the boss's stats
    let mut boss_hit_points = 0;
    let mut boss_damage = 0;
    let mut boss_armor = 0;
    for line in input.lines() {
        let mut colon_iter = line.split(": ");
        let stat_name = colon_iter.next().expect("Line should have a named stat");
        let stat_value = colon_iter
            .next()
            .expect("Line should have a stat value")
            .parse()
            .expect("Stat value should be a number");
        match stat_name {
            "Hit Points" => boss_hit_points = stat_value,
            "Damage" => boss_damage = stat_value,
            "Armor" => boss_armor = stat_value,
            _ => panic!("Stat should be one of \"Hit Points\", \"Damage\", or \"Armor\""),
        }
    }
    let boss = Fighter {
        hit_points: boss_hit_points,
        damage: boss_damage,
        armor: boss_armor,
    };

    // Use the recursive functions to find the minimum cost of equipment that enables the player to
    // win.
    let mut purchased_equipment = Vec::with_capacity(4);
    let minimum_victory_cost = purchase_weapon(&mut purchased_equipment, &boss);

    Solution::U32(minimum_victory_cost)
}

// The puzzle description provides no examples for this puzzle.
