use itertools::Itertools;

#[derive(Clone)]
struct Character {
    hp: i32,
    armor: i32,
    damage: i32,
}

struct Equipment {
    price: i32,
    armor: i32,
    damage: i32,
}


fn do_combat(mut player: Character, mut boss: Character) -> bool {
    while player.hp > 0 {
        boss.hp -= (player.damage - boss.armor).max(1);
        if boss.hp <= 0 { return true; }
        player.hp -= (boss.damage - player.armor).max(1);
    }
    false
}


fn main() {
    let boss = Character { hp: 104, damage: 8, armor: 1 };

    let weapons = vec![
        Equipment { price: 8, damage: 4, armor: 0 },
        Equipment { price: 10, damage: 5, armor: 0 },
        Equipment { price: 25, damage: 6, armor: 0 },
        Equipment { price: 40, damage: 7, armor: 0 },
        Equipment { price: 74, damage: 8, armor: 0 },
    ];

    let armors = vec![
        Equipment { price: 13, damage: 0, armor: 1 },
        Equipment { price: 31, damage: 0, armor: 2 },
        Equipment { price: 53, damage: 0, armor: 3 },
        Equipment { price: 75, damage: 0, armor: 4 },
        Equipment { price: 102, damage: 0, armor: 5 },
    ];

    let rings = vec![
        Equipment { price: 25, damage: 1, armor: 0 },
        Equipment { price: 50, damage: 2, armor: 0 },
        Equipment { price: 100, damage: 3, armor: 0 },
        Equipment { price: 20, damage: 0, armor: 1 },
        Equipment { price: 40, damage: 0, armor: 2 },
        Equipment { price: 80, damage: 0, armor: 3 },
    ];

    let mut minimum_cost = i32::MAX;
    let mut max_cost = i32::MIN;

    for weapon in weapons {
        for i in 0..=armors.len() {
            for armor in armors.iter().combinations(i) {
                for j in 0..=2 {
                    for ring in rings.iter().combinations(j) {
                        let mut damage = weapon.damage;
                        let mut arm = weapon.armor;
                        let mut cost = weapon.price;
                        armor.iter().for_each(|&a| {
                            arm += a.armor;
                            cost += a.price
                        });
                        ring.iter().for_each(|&a| {
                            arm += a.armor;
                            cost += a.price;
                            damage += a.damage
                        });
                        if do_combat(Character{damage, armor:arm, hp: 100}, boss.clone()) {
                            minimum_cost = minimum_cost.min(cost);
                        }
                        else {
                            max_cost = max_cost.max(cost);
                        }
                    }
                }
            }
        }
    }

    println!("{} {}", minimum_cost, max_cost);
}