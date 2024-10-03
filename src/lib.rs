use rand::seq::SliceRandom;
use rand::Rng;

pub type Condition = u8;
pub type Emotion = u8;
pub type Sequence = Vec<(Condition, Emotion)>;

// TODO: don't write by hand
const N_COND: u8 = 2 * 3;
const N_EMOTE: u8 = 5;

pub fn generate_seq<R: Rng>(rng: &mut R) -> Sequence {
    let mut groups: Vec<Vec<(Condition, Emotion)>> = vec![vec![]; N_EMOTE as usize];
    for c in 0..N_COND {
        let dealt = deal_emote_to_groups(rng);
        for g in 0..N_EMOTE as usize {
            groups[g].push((c, dealt[g]));
        }
    }
    for g in &mut groups {
        println!("↓{:?}", g);
        g.shuffle(rng);
        println!("↑{:?}", g);
    }
    let seq: Sequence = groups.into_iter().flatten().collect();
    assert!(seq.len() == (N_EMOTE * N_COND) as usize);
    seq
}

fn deal_emote_to_groups<R: Rng>(rng: &mut R) -> Vec<Emotion> {
    let mut cards: Vec<u8> = (0..N_EMOTE as Emotion).collect();
    cards.shuffle(rng);
    cards
}
