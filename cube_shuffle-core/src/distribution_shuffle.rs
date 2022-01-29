use std::collections::HashMap;
use std::hash::Hash;

use rand::{Rng, RngCore};
use rand::prelude::SliceRandom;

pub type Odds = f64;

#[derive(Clone, Debug, Copy)]
pub struct Pile {
    cards: u32,
    randomness: Odds,
}

#[derive(Clone, Debug)]
pub struct Pack<P> {
    card_sources: HashMap<P, u32>,
}

pub fn shuffle<'a, P>(
    piles: &'a HashMap<P, Pile>,
    pack_size: u32,
    random: &mut impl RngCore)
    -> Vec<Pack<&'a P>>
    where P: Eq + Hash
{
    if pack_size == 0 {
        panic!("TODO");
    }
    let card_count: u32 = piles.values().map(|p| p.cards).sum();
    let pack_count: u32 = card_count / pack_size;
    if pack_count == 0 {
        panic!("TODO");
    }
    let pack_overflow: u32 = card_count % pack_size;
    if pack_overflow != 0 {
        panic!("TODO");
    }

    let mut packs: Vec<HashMap<Option<&P>, u32>> = Vec::new();
    for _ in 0..pack_count {
        packs.push(HashMap::new())
    }

    let mut randomized: Vec<&P> = Vec::new();
    for (pile_name, pile) in piles {
        for c in 0..pile.cards {
            let skip: bool = random.gen_bool(pile.randomness);
            if skip {
                randomized.push(&pile_name);
            }

            let pack_index: usize = (c % pack_count) as usize;
            *packs[pack_index]
                .entry(if skip { None } else { Some(&pile_name) })
                .or_insert(0) += 1;
        }

        packs.shuffle(random);
        packs.sort_by(|a, b|
            a.values()
                .sum::<u32>()
                .cmp(
                    &b.values()
                        .sum::<u32>()
                )
        );
    }

    randomized.shuffle(random);
    let finalized_packs: Vec<Pack<&P>> = packs.iter().map(|incomplete_pack| {
        let mut card_sources: HashMap<&P, u32> =
            incomplete_pack.iter()
            .filter_map(|(source, amount)| {
                match *source {
                    None => { None }
                    Some(s) => { Some((s, *amount)) }
                }
            }).collect();

        let randomized_picks = incomplete_pack.get(&None).unwrap_or(&0);
        for _ in 0..*randomized_picks {
            let card_source = randomized.pop().unwrap();
            *card_sources
                .entry(card_source)
                .or_insert(0) += 1;
        }
        Pack {
            card_sources
        }
    }).collect();

    return finalized_packs;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::SystemTime;
    use rand::prelude::SliceRandom;
    use proptest::collection::hash_map;
    use proptest::prelude::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use crate::distribution_shuffle::{Odds, Pile, shuffle};

    prop_compose! {
        fn arb_odds()(odds in 0f64..=1f64) -> Odds{
            odds
        }
    }

    prop_compose! {
        fn arb_pile
            (min_cards:u32, max_cards: u32)
            (cards in min_cards..max_cards, odds in arb_odds())
            -> Pile {
            Pile {
                cards: cards,
                randomness: odds
            }
        }
    }

    prop_compose! {
        fn arb_piles
            ()
            (piles in hash_map(any::<String>(), arb_pile(1, 10_000), 1..100))
            -> HashMap<String, Pile>{
            piles
        }
    }

    fn get_valid_pack_sizes(cards:u32) -> Vec<u32> {
        (1..=cards).filter(move |d| {cards % d == 0}).collect()
    }

    proptest! {
        #[test]
        fn shuffled_cards (
            piles in arb_piles(),
            seed in any::<u64>()
        ){
            let mut rng = StdRng::seed_from_u64(seed);
            let total_card_count:u32 = piles.values().map(|p| p.cards).sum();
            println!("Card count={}", total_card_count);
            let pack_sizes = get_valid_pack_sizes(total_card_count);
            println!("Possible pack sizes={:?}", pack_sizes);
            let pack_size = *pack_sizes.choose(&mut rng).unwrap();
            println!("Pack size={}", pack_size);

            let start_time = SystemTime::now();
            let shuffled = shuffle(&piles, pack_size, &mut rng);
            match start_time.elapsed() {
                Ok(elapsed) => {println!("Shuffling took {} seconds", elapsed.as_secs())}
                Err(e) => {println!("Shuffling time measurement failed: {:?}", e)}
            }

            let card_sources_count:Vec<u32> =
                shuffled.iter()
                .flat_map(|p| {p.card_sources.values()})
                .map(|x| {*x})
                .collect();

            // All card sources must be positive
            assert!(card_sources_count.iter().all(|c| {*c > 0}));

            // Same number of total cards
            assert_eq!(total_card_count, card_sources_count.iter().sum());

            // All packs most be requested size
            for pack in shuffled {
                assert_eq!(pack_size, pack.card_sources.values().sum());
            }
        }
    }
}
