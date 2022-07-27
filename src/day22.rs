mod day21;

use std::collections::HashMap;
use crate::day21::Character;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Effect {
    Poison,
    Shield,
    Recharge,
}

#[derive(Eq, Hash, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

static mut MINIMUM: i32 = i32::MAX;


unsafe fn do_turn(spells: &HashMap<Spell, i32>,
                  mut player: Character,
                  mut boss: Character,
                  mut effects: HashMap<Effect, i32>,
                  mut mana: i32,
                  mana_spent: i32) -> i32 {
    let mut best = i32::MAX;
    // These are for second part
    player.hp -= 1;
    if player.hp == 0 { return best };
    for (k, v) in effects.iter_mut() {
        if v == &0 { continue; };
        match k {
            Effect::Poison => boss.hp -= 3,
            Effect::Shield => if v == &1 { player.armor = 0 },
            Effect::Recharge => mana += 101
        }
        if boss.hp <= 0 { return mana_spent; }
        *v -= 1;
    }
    for (spell, mana_cost) in spells {
        if mana_cost > &mana { continue; };
        let mut temp: HashMap<Effect, i32> = HashMap::new();
        for (k, v) in effects.iter() {
            temp.insert(*k, *v);
        }
        best = best.min(cast_n_shit(spells, player.clone(), boss.clone(), temp, mana, mana_spent, spell, mana_cost));
    }

    best
}

unsafe fn cast_n_shit(spells: &HashMap<Spell, i32>, mut player: Character, mut boss: Character, mut effects: HashMap<Effect, i32>, mut mana: i32, mut mana_spent: i32, spell: &Spell, mana_cost: &i32) -> i32 {
    if mana_spent >= MINIMUM {
        return mana_spent;
    }
    match spell {
        Spell::MagicMissile => boss.hp -= 4,
        Spell::Drain => {
            boss.hp -= 2;
            player.hp += 2
        }
        Spell::Shield => {
            if effects[&Effect::Shield] > 0
            {
                return i32::MAX;
            } else {
                *effects.entry(Effect::Shield).or_insert(0) = 6
            }
            player.armor = 7;
        }
        Spell::Poison => {
            if effects[&Effect::Poison] > 0 {
                return i32::MAX;
            } else {
                *effects.entry(Effect::Poison).or_insert(0) = 6
            }
        }
        Spell::Recharge => {
            if effects[&Effect::Recharge] > 0 {
                return i32::MAX;
            } else {
                *effects.entry(Effect::Recharge).or_insert(0) = 5
            }
        }
    }
    mana -= mana_cost;
    mana_spent += mana_cost;
    if boss.hp <= 0 {
        unsafe {
            MINIMUM = MINIMUM.min(mana_spent);
        }
        println!("killed boss {}", mana_spent);
        return mana_spent;
    }
    for (k, v) in effects.iter_mut() {
        if v == &0 { continue; };
        match k {
            Effect::Poison => boss.hp -= 3,
            Effect::Shield => if v == &1 { player.armor = 0 },
            Effect::Recharge => mana += 101
        }
        if boss.hp <= 0 {
            unsafe {
                MINIMUM = MINIMUM.min(mana_spent);
            }
            println!("killed boss {}", mana_spent);
            return mana_spent;
        }
        *v -= 1;
    }
    player.hp -= boss.damage - player.armor;
    if player.hp <= 0 { return i32::MAX; }
    let temp = do_turn(spells,
                       player.clone(),
                       boss.clone(),
                       effects.into_iter().collect(),
                       mana,
                       mana_spent);
    temp
}


fn main() {
    let boss = Character { hp: 51, armor: 0, damage: 9 };

    let mut spell_costs = HashMap::new();
    spell_costs.insert(Spell::MagicMissile, 53);
    spell_costs.insert(Spell::Drain, 73);
    spell_costs.insert(Spell::Shield, 113);
    spell_costs.insert(Spell::Poison, 173);
    spell_costs.insert(Spell::Recharge, 229);

    let mut effect = HashMap::new();
    effect.insert(Effect::Poison, 0);
    effect.insert(Effect::Recharge, 0);
    effect.insert(Effect::Shield, 0);
    unsafe {
        println!("{}",
                 do_turn(&spell_costs, Character { hp: 50, armor: 0, damage: 0 }, boss, effect, 500, 0)
        )
    }
}