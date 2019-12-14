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

pub fn part1(input: &str) -> impl Display {
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

    let fuel_reaction = reacts
        .get(&Element {
            amount: 1,
            name: "FUEL".to_owned(),
        })
        .unwrap();

    let mut stack: VecDeque<Element> = VecDeque::new();
    for fuel_input in fuel_reaction {
        stack.push_back(fuel_input.clone());
    }

    // let mut final_formula = vec![];
    let mut final_formula = HashMap::new();
    let mut stash = HashMap::new();

    while !stack.is_empty() {
        println!("-----");
        let mut needed = stack.pop_front().unwrap();
        println!("needed_element {:?}", needed);
        if is_basic(&reacts, &needed) {
            let entry = final_formula.entry(needed.name).or_insert(0);
            *entry += needed.amount;
            continue;
        }

        let (base_reaction, ingredients) = reacts.get_key_value(&needed).unwrap();
        println!("base_reaction {:?}", base_reaction);

        let stash_amount = stash.entry(needed.name.clone()).or_insert(0);
        if *stash_amount >= needed.amount {
            *stash_amount -= needed.amount;
            println!("{} taken from stash", needed.amount);
            continue;
        } else if *stash_amount > 0 {
            needed.amount -= *stash_amount;
            *stash_amount = 0;
        }

        let reaction_number_needed = round_division(needed.amount, base_reaction.amount);
        let produced_amount = reaction_number_needed * base_reaction.amount;
        if produced_amount > needed.amount {
            println!(
                "putting {} of {} in stash",
                produced_amount - needed.amount,
                needed.name
            );
            *stash_amount += produced_amount - needed.amount;
        }

        for ingredient in ingredients {
            let ingredient_needed_amount = ingredient.amount * reaction_number_needed;
            let new_name = ingredient.name.clone();

            println!(
                "ingredient {:?} ingredient_needed_amount {} new_name {}",
                ingredient, ingredient_needed_amount, new_name
            );
            stack.push_back(Element {
                amount: ingredient_needed_amount,
                name: new_name,
            });
        }
    }
    print_formula(&final_formula);

    let mut total_ore = 0;
    for (element_name, amount) in final_formula {
        let (el, ore) = reacts
            .get_key_value(&Element {
                amount: amount,
                name: element_name,
            })
            .unwrap();
        let more_ore = round_division(amount, el.amount) * ore[0].amount;
        println!("{} more ore for {}", more_ore, el.name);
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

fn print_formula(formula: &HashMap<String, usize>) {
    for (element, amount) in formula {
        println!("{} {} +", amount, element)
    }
    println!("=> 1 FUEL");
    // println!(
    //     "{} => 1 FUEL",
    //     formula
    //         .iter()
    //         .map(|el| format!("{} {}", el.amount, el.name))
    //         .collect::<Vec<String>>()
    //         .join(" + ")
    // )
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
    0
}
