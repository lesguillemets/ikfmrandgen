mod consts;

use consts::SIMPLE_SEQ;
use rand::seq::SliceRandom;
use rand::Rng;

pub type Condition = u8;
pub type Emotion = u8;
pub type Sequence = Vec<(Condition, Emotion)>;

// TODO: don't write by hand
const N_COND: u8 = 2 * 3;
const N_EMOTE: u8 = 5;

const DEBUG: bool = true;

pub fn generate_seq<R: Rng>(rng: &mut R) -> Sequence {
    let mut groups: Vec<Vec<(Condition, Emotion)>> = vec![vec![]; N_EMOTE as usize];
    for c in 0..N_COND {
        let dealt = deal_emote_to_groups(rng);
        for g in 0..N_EMOTE as usize {
            groups[g].push((c, dealt[g]));
        }
    }
    for g in &mut groups {
        if DEBUG {
            println!("↓{:?}", g);
        }
        g.shuffle(rng);
        if DEBUG {
            println!("↑{:?}", g);
        }
    }
    let seq: Sequence = groups.into_iter().flatten().collect();
    assert_eq!(seq.len(), (N_EMOTE * N_COND) as usize);
    {
        let mut validate: Sequence = seq.clone();
        validate.sort_by(|(c0, e0), (c1, e1)| (e0, c0).cmp(&(e1, c1)));
        assert_eq!(&validate, &SIMPLE_SEQ);
    }
    seq
}

fn deal_emote_to_groups<R: Rng>(rng: &mut R) -> Vec<Emotion> {
    let mut cards: Vec<u8> = (0..N_EMOTE as Emotion).collect();
    cards.shuffle(rng);
    cards
}
