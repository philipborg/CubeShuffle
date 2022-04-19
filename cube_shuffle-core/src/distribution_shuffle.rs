use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use parse_display::{Display, FromStr};
use rand::prelude::SliceRandom;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

use crate::distribution_shuffle::ShufflingErrors::{CardOverflow, EmptyPacks};

pub type Odds = f64;

#[derive(Clone, Debug, Copy, PartialEq, Display, FromStr, Serialize, Deserialize)]
#[display("{cards}:{randomness}")]
pub struct Pile {
    pub cards: usize,
    pub randomness: Odds,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pack<P>
where
    P: Hash + Eq + Serialize,
{
    pub card_sources: HashMap<P, usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ShufflingErrors {
    EmptyPacks,
    CardOverflow {
        current_cards: u128,
        max_cards: u128,
    },
}

pub fn shuffle<'a, P>(
    piles: &'a HashMap<P, Pile>,
    pack_size: usize,
    random: &mut impl RngCore,
) -> Result<Vec<Pack<&'a P>>, ShufflingErrors>
where
    P: Eq + Hash + Serialize,
{
    if pack_size == 0 {
        return Err(EmptyPacks);
    }

    let card_count: u128 = piles.values().map(|p| p.cards as u128).sum();

    if card_count > usize::MAX as u128 {
        return Err(CardOverflow {
            current_cards: card_count,
            max_cards: usize::MAX as u128,
        });
    }

    let pack_count: usize = card_count as usize / pack_size;

    let pack_overflow: usize = card_count as usize % pack_size;
    let overflow_cards: HashSet<usize> = if pack_overflow == 0 {
        HashSet::new()
    } else {
        let mut cards: Vec<usize> = (0..(card_count as usize)).collect();
        cards.shuffle(random);
        cards.into_iter().take(pack_overflow as usize).collect()
    };

    let mut packs: Vec<HashMap<Option<&P>, usize>> = Vec::new();
    for _ in 0..pack_count {
        packs.push(HashMap::new())
    }

    let mut card_index: usize = 0;
    let mut randomized: Vec<&P> = Vec::new();
    for (pile_name, pile) in piles {
        let mut pile_modifier: usize = 0;
        for c in 0..pile.cards {
            if overflow_cards.contains(&card_index) {
                card_index += 1;
                pile_modifier += 1;
                continue;
            }
            card_index += 1;
            let skip: bool = random.gen_bool(pile.randomness);
            if skip {
                randomized.push(pile_name);
            }

            let pack_index: usize = (c - pile_modifier) % pack_count as usize;
            *packs[pack_index]
                .entry(if skip { None } else { Some(pile_name) })
                .or_insert(0) += 1;
        }

        packs.shuffle(random);
        packs.sort_by_key(|k| k.values().sum::<usize>());
    }

    randomized.shuffle(random);
    let finalized_packs: Vec<Pack<&P>> = packs
        .iter()
        .map(|incomplete_pack| {
            let mut card_sources: HashMap<&P, usize> = incomplete_pack
                .iter()
                .filter_map(|(source, amount)| (*source).map(|s| (s, *amount)))
                .collect();

            let randomized_picks = incomplete_pack.get(&None).unwrap_or(&0);
            for _ in 0..*randomized_picks {
                let card_source = randomized.pop().unwrap();
                *card_sources.entry(card_source).or_insert(0) += 1;
            }
            Pack { card_sources }
        })
        .collect();

    Ok(finalized_packs)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::SystemTime;

    use proptest::collection::hash_map;
    use proptest::prelude::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::distribution_shuffle::ShufflingErrors::EmptyPacks;
    use crate::distribution_shuffle::{shuffle, Odds, Pile};

    prop_compose! {
        fn arb_odds()(odds in 0f64..=1f64) -> Odds{
            odds
        }
    }

    prop_compose! {
        fn arb_pile
            (min_cards: usize, max_cards: usize)
            (cards in min_cards..max_cards, odds in arb_odds())
            -> Pile {
            Pile {
                cards,
                randomness: odds
            }
        }
    }

    prop_compose! {
        fn arb_piles
            ()
            (piles in hash_map(any::<String>(), arb_pile(0, 1_000), 0..100))
            -> HashMap<String, Pile>{
            piles
        }
    }

    proptest! {
        #[test]
        fn shuffled_cards (
            piles in arb_piles(),
            seed in any::<u64>(),
        ){
            println!("Piles={}", piles.values().count());
            let mut rng = StdRng::seed_from_u64(seed);
            let total_card_count:usize = piles.values().map(|p| p.cards).sum();
            println!("Card count={}", total_card_count);
            let pack_size = rng.gen_range(1..=(if total_card_count == 0 {usize::MAX} else {total_card_count}));
            println!("Pack size={}", pack_size);

            let start_time = SystemTime::now();
            let shuffled = shuffle(&piles, pack_size, &mut rng).unwrap();
            match start_time.elapsed() {
                Ok(elapsed) => {println!("Shuffling took {} seconds", elapsed.as_secs())}
                Err(e) => {println!("Shuffling time measurement failed: {:?}", e)}
            }

            let card_sources_count:Vec<usize> =
                shuffled.iter()
                .flat_map(|p| {p.card_sources.values()})
                .copied()
                .collect();

            // All card sources must be positive
            assert!(card_sources_count.iter().all(|c| {*c > 0}));

            // Total cards should equal expected filled packs sum
            assert_eq!((total_card_count / pack_size) * pack_size, card_sources_count.iter().sum());

            // All packs most be requested size
            for pack in shuffled {
                assert_eq!(pack_size, pack.card_sources.values().sum());
            }
        }

        #[test]
        fn empty_packs(
            piles in arb_piles(),
            seed in any::<u64>(),
        ){
            let mut rng = StdRng::seed_from_u64(seed);
            let shuffled = shuffle(&piles, 0, &mut rng);
            assert_eq!(Err(EmptyPacks), shuffled);
        }
    }
}
