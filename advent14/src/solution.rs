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
    let reacts = parse(input);
    produce(
        &reacts,
        Element {
            amount: 1,
            name: "FUEL".to_owned(),
        },
    )
}

pub fn part2(input: &str) -> impl Display {
    let reacts = parse(input);
    let start_ore: usize = 1000000000000;

    let mut max_attempt = start_ore;
    let mut min_attempt = 0;
    loop {
        let candidate = min_attempt + (max_attempt - min_attempt) / 2;
        let total_ore_used = produce(
            &reacts,
            Element {
                amount: candidate,
                name: "FUEL".to_owned(),
            },
        );
        if min_attempt == candidate {
            return candidate;
        }
        if total_ore_used > start_ore {
            max_attempt = candidate;
        } else {
            min_attempt = candidate;
        }
    }
}

fn produce(reacts: &HashMap<Element, Vec<Element>>, element: Element) -> usize {
    let mut stack: VecDeque<Element> = VecDeque::new();
    let mut stash = HashMap::new();

    stack.push_back(element);
    let mut total_ore = 0;

    while !stack.is_empty() {
        // get next ingredient needed, if it's ORE we can just add it
        let mut needed = stack.pop_front().unwrap();
        if needed.name == "ORE" {
            total_ore += needed.amount;
            continue;
        }

        // get the recipe and the base reaction, and check what we have in the stash
        let (base_reaction, ingredients) = reacts.get_key_value(&needed).unwrap();
        let stash_amount = stash.entry(needed.name.clone()).or_insert(0);
        if *stash_amount >= needed.amount {
            *stash_amount -= needed.amount;
            continue;
        } else if *stash_amount > 0 {
            needed.amount -= *stash_amount;
            *stash_amount = 0;
        }

        // if we don't have enough in the stash we'll figure out how much we
        // need to make after accounting for the stash and if we end up making
        // extra we'll put the extra in the stash and the rest will be used away
        let reaction_number_needed = round_division(needed.amount, base_reaction.amount);
        let produced_amount = reaction_number_needed * base_reaction.amount;
        if produced_amount > needed.amount {
            *stash_amount += produced_amount - needed.amount;
        }

        // Now that we know how much to make, we make it and so we find the
        // amount of ingredients that are needed and add them to the stack that
        // we need to make. The amount is obviouly dependent on the number of
        // reactions needed to deliver them.
        for ingredient in ingredients {
            let ingredient_needed_amount = ingredient.amount * reaction_number_needed;
            let new_name = ingredient.name.clone();
            stack.push_back(Element {
                amount: ingredient_needed_amount,
                name: new_name,
            });
        }
    }
    total_ore
}

fn round_division(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return a / b;
    }
    a / b + 1
}

fn parse(input: &str) -> HashMap<Element, Vec<Element>> {
    // just to be clear here... the hashing is technically incorrect for this
    // hashmap, but it works because in the input you can only find one way to
    // product that type of output. So it hashes on the element name, and the
    // amount is considered to be that of the basic form.
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
    reacts
}
