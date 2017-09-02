use super::*;
use std::cmp::{min, max, Ordering};

/// [0..1] should probably be somewhere close to 0.1
const MAX_DIFFICULTY_DELTA: f32 = 0.2;

/// Allows for evaluation of content for inclusion to dungeon.
pub trait Evaluate: Clone {
    fn theme(&self) -> &[Keyword];
    fn difficulty(&self) -> usize;
    fn max_difficulty(&self) -> usize {
        10
    }
    fn normalized_difficulty(&self) -> f32 {
        self.difficulty() as f32 / self.max_difficulty() as f32
    }
}

pub fn rank_by_fitness<'a, T: Evaluate, K: AsRef<Keyword>>(
    content: &'a [T],
    room_difficulty: f32,
    room_theme: &[K])
    -> Vec<(f32, &'a T)> {
    let mut fitnesses: Vec<(f32, &T)> = content.iter().enumerate()
        .map(|(i, ref item)| {
            (evaluate(*item, room_difficulty, room_theme), &content[i])
        }).collect();
    fitnesses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Less));
    fitnesses
}

pub fn rank_by_difficulty<'a, T: Evaluate>(
    content: &'a [T],
    room_difficulty: f32)
    -> Vec<(f32, &'a T)> {
    let mut fitnesses: Vec<(f32, &T)> = content.iter().enumerate()
        .map(|(i, ref item)| {
            (evaluate_difficulty(*item, room_difficulty), &content[i])
        }).collect();
    fitnesses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Less));
    fitnesses
}

/// Returns the compatibility of content with the given keywords and difficulty. Basically a fitness function.
pub fn evaluate<T: Evaluate, K: AsRef<Keyword>>(content: &T, room_difficulty: f32, room_theme: &[K]) -> f32 {
    // Theme suitability, [0..1]
    let theme_match_frac = evaluate_theme(content, room_theme);

    // Difficulty suitability [0..1]
    let difficulty_match_frac = evaluate_difficulty(content, room_difficulty);

    let fitness = theme_match_frac.min(difficulty_match_frac);
    return fitness;
}

pub fn evaluate_theme<T: Evaluate, K: AsRef<Keyword>>(content: &T, room_theme: &[K]) -> f32 {
    // Count number of keywords in content that match with room
    let num_matching_keywords = content.theme().iter().filter(|&content_kw| room_theme.iter().any(|room_kw| room_kw.as_ref() == content_kw)).count() as f32;
    //let num_content_keywords = content.theme().iter().count() as f32;
    let num_room_keywords = room_theme.iter().count() as f32;

    // Theme suitability, [0..1]
    num_matching_keywords / num_room_keywords
}

pub fn evaluate_difficulty<T: Evaluate>(content: &T, room_difficulty: f32) -> f32 {
    let delta = (content.normalized_difficulty() - room_difficulty).abs();
    match MAX_DIFFICULTY_DELTA - delta {
        distance if distance > 0. => {distance / MAX_DIFFICULTY_DELTA},
        _ => 0.,
    }
}
