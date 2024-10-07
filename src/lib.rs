mod consts;

use consts::SIMPLE_SEQ;
use rand::seq::SliceRandom;
use rand::Rng;

use std::collections::HashSet;

pub type Condition = u8;
pub type Emotion = u8;
pub type Sequence = Vec<(Condition, Emotion)>;

// TODO: don't write by hand
const N_COND: u8 = 2 * 3;
const N_EMOTE: u8 = 5;

const DEBUG: bool = false;

const HEADER: &str = "index,condition1,condition2,layout,emotion,procedure\n";
#[allow(clippy::erasing_op)]
#[allow(clippy::identity_op)]
const TUTORIAL: [(Condition, Emotion); 3] = [((2 * 1 + 1), 6), ((2 * 2 + 0), 7), ((2 * 0 + 1), 8)];

pub fn generate_and_filter<R: Rng>(rng: &mut R) -> Sequence {
    let mut c = 0;
    loop {
        c += 1;
        let seq = generate_seq(rng);
        if succ_times(&seq) <= 2 && maximum_succ(&seq) < 3 && minimum_exps(&seq) >= 4 {
            eprintln!("found one after {} generations", c);
            return seq;
        }
    }
}

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
    {
        // when sorted, this resulting sequence is the
        // same as the SIMPLE_SEQ.
        let mut validate: Sequence = seq.clone();
        validate.sort_by(|(c0, e0), (c1, e1)| (e0, c0).cmp(&(e1, c1)));
        assert_eq!(&validate, &SIMPLE_SEQ);
    }
    seq
}

pub fn to_csv<R: Rng>(rng: &mut R, s: Sequence) -> String {
    let length = s.len() + TUTORIAL.len();
    let mut layouts: Vec<u8> = vec![0; length / 2];
    layouts.extend(vec![1; length - length / 2]);
    layouts.shuffle(rng);
    let layouts = layouts;
    let mut text = String::from(HEADER);
    let mut index = 0;
    for (&(c, e), &l) in (TUTORIAL.iter()).chain(&s).zip(&layouts) {
        index += 1;
        let (vf, re) = cond_to_pair(c);
        let procedure = if index <= TUTORIAL.len() { 0 } else { 1 };
        let line = [index, vf.into(), re.into(), l.into(), e.into(), procedure]
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<_>>()
            .join(",");
        text.push_str(&line);
        text.push('\n');
    }
    text
}

fn deal_emote_to_groups<R: Rng>(rng: &mut R) -> Vec<Emotion> {
    let mut cards: Vec<u8> = (0..N_EMOTE as Emotion).collect();
    cards.shuffle(rng);
    cards
}

fn cond_to_pair(c: Condition) -> (Condition, Condition) {
    (c / 2, c % 2)
}

pub fn print_seq(s: Sequence) {
    println!("VF\tRef\tEmotion");
    for (c, e) in s {
        let (vf, re) = cond_to_pair(c);
        println!("{}\t{}\t{}", vf, re, e);
    }
}

/// VALIDATORS

fn succ_times(s: &Sequence) -> usize {
    // number of times when the emotion seen in a trial is the same
    // as the emotion seen in last trial
    s.iter()
        .zip(&s[1..])
        .map(|((_, e0), (_, e1))| if e0 == e1 { 1 } else { 0 })
        .sum()
}

fn maximum_succ(s: &Sequence) -> usize {
    // the longest streak where the same expression is seen successively
    let mut m = 0;
    let mut current_streak = 0;
    let mut current_emotion = None;
    for &(_, e) in s {
        if Some(e) == current_emotion {
            current_streak += 1;
        } else {
            // start of another streak
            current_emotion = Some(e);
            m = m.max(current_streak);
            current_streak = 1;
        }
    }
    m
}

fn minimum_exps(s: &Sequence) -> usize {
    let mut minimum = usize::MAX;
    for group in s.chunks(N_COND as usize) {
        let emotions_in_group = group
            .iter()
            .map(|d| d.1)
            .collect::<HashSet<Emotion>>()
            .len();
        minimum = minimum.min(emotions_in_group);
    }
    minimum
}
