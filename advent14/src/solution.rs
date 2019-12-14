use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone)]
struct Element {
    amount: usize,
    name: String,
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        self.name == other.name
    }
}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Element {
    fn from_str(input: &str) -> Self {
        let comp = input.trim().split(" ").collect::<Vec<&str>>();
        Element {
            amount: comp[0].parse::<usize>().unwrap(),
            name: comp[1].to_owned(),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut reacts = HashMap::new();
    for line in input.lines() {
        let comp = line.trim().split(" => ").collect::<Vec<&str>>();
        let formula = comp[0]
            .trim()
            .split(",")
            .map(|x| Element::from_str(x))
            .collect::<Vec<Element>>();
        reacts.insert(Element::from_str(comp[1]), formula);
    }

    let mut stash = HashMap::new();
    produce_fuel(
        &reacts,
        &mut stash,
        &Element {
            amount: 1,
            name: "FUEL".to_owned(),
        },
    )
}

fn produce_fuel(
    reacts: &HashMap<Element, Vec<Element>>,
    stash: &mut HashMap<String, usize>,
    element: &Element,
) -> usize {
    let mut stack: VecDeque<Element> = VecDeque::new();
    let mut final_formula = HashMap::new();
    let fuel_reaction = reacts.get(&element).unwrap();

    for fuel_input in fuel_reaction {
        stack.push_back(Element {
            amount: fuel_input.amount * element.amount,
            name: fuel_input.name.clone(),
        });
    }

    while !stack.is_empty() {
        let mut needed = stack.pop_front().unwrap();
        if is_basic(&reacts, &needed) {
            let entry = final_formula.entry(needed.name).or_insert(0);
            *entry += needed.amount;
            continue;
        }

        let (base_reaction, ingredients) = reacts.get_key_value(&needed).unwrap();
        let stash_amount = stash.entry(needed.name.clone()).or_insert(0);
        if *stash_amount >= needed.amount {
            *stash_amount -= needed.amount;
            continue;
        } else if *stash_amount > 0 {
            needed.amount -= *stash_amount;
            *stash_amount = 0;
        }

        let reaction_number_needed = round_division(needed.amount, base_reaction.amount);
        let produced_amount = reaction_number_needed * base_reaction.amount;
        if produced_amount > needed.amount {
            *stash_amount += produced_amount - needed.amount;
        }

        for ingredient in ingredients {
            let ingredient_needed_amount = ingredient.amount * reaction_number_needed;
            let new_name = ingredient.name.clone();
            stack.push_back(Element {
                amount: ingredient_needed_amount,
                name: new_name,
            });
        }
    }

    let mut total_ore = 0;
    for (element_name, amount) in final_formula {
        let (el, ore) = reacts
            .get_key_value(&Element {
                amount: amount,
                name: element_name,
            })
            .unwrap();
        let more_ore = round_division(amount, el.amount) * ore[0].amount;
        total_ore += more_ore;
    }
    total_ore
}

fn round_division(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return a / b;
    }
    a / b + 1
}

fn is_basic(reacts: &HashMap<Element, Vec<Element>>, e: &Element) -> bool {
    let formula = reacts.get(e).unwrap();
    if formula.len() != 1 {
        return false;
    }
    if formula[0].name != "ORE" {
        return false;
    }
    true
}

pub fn part2(input: &str) -> impl Display {
    let mut reacts = HashMap::new();
    for line in input.lines() {
        let comp = line.trim().split(" => ").collect::<Vec<&str>>();
        let formula = comp[0]
            .trim()
            .split(",")
            .map(|x| Element::from_str(x))
            .collect::<Vec<Element>>();
        reacts.insert(Element::from_str(comp[1]), formula);
    }

    let first_fuel = part1(input);
    let fuel_reaction = reacts
        .get(&Element {
            amount: 1,
            name: "FUEL".to_owned(),
        })
        .unwrap();

    let start_ore: usize = 1000000000000;

    let mut max_attempt = start_ore;
    let mut min_attempt = 0;
    loop {
        let candidate = min_attempt + (max_attempt - min_attempt) / 2;
        let total_ore_used = produce_fuel(
            &reacts,
            &mut HashMap::new(),
            &Element {
                amount: candidate,
                name: "FUEL".to_owned(),
            },
        );
        let total_ore_used_and_one = produce_fuel(
            &reacts,
            &mut HashMap::new(),
            &Element {
                amount: candidate + 1,
                name: "FUEL".to_owned(),
            },
        );
        if total_ore_used_and_one > start_ore && total_ore_used < start_ore {
            return candidate;
        }
        if total_ore_used > start_ore {
            max_attempt = candidate;
        } else {
            min_attempt = candidate;
        }
    }
}
