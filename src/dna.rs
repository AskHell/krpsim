use std::collections::HashMap;

use crate::score::Score;
use crate::solver::{Path, Step};

pub struct DNA {
  path: Path,
  favorite_steps: Vec<Step>,
  probability: f32,
}

fn is_in_vec<T: PartialEq>(value: &T, vec: &Vec<&T>) -> bool {
  vec
    .clone()
    .into_iter()
    .fold(false, |acc, curr| acc || curr == value)
}

fn build_favorite_steps(path: &Path) -> Vec<Step> {
  let map = path.iter().fold(HashMap::new(), |mut acc, current_step| {
    let count = acc.get(&current_step).unwrap_or(&0);
    acc.insert(current_step, *count + 1);
    acc
  });
  let mut step_counts: Vec<(&Step, usize)> = map.into_iter().collect();
  step_counts.sort_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b));
  step_counts
    .into_iter()
    .map(|(value, _)| value.clone())
    .collect()
}

impl DNA {
  pub fn new(path: Path, score: Score, len: usize) -> Self {
    let favorite_steps = build_favorite_steps(&path);
    let probability = score as f32 / len as f32;
    Self {
      path,
      favorite_steps,
      probability,
    }
  }
  // available_steps cannot be empty
  pub fn pick_steps(&self, index: usize, available_steps: &Vec<&Step>) -> Vec<Step> {
    let step = self.pick_step(index, available_steps);
    // TODO: no magic number
    let n = (self.probability * 100.).floor() as usize;
    (0..n).map(|_| step.clone()).collect()
  }

  // TODO: memoize
  fn pick_step(&self, index: usize, available_steps: &Vec<&Step>) -> Step {
    match self.path.get(index) {
      Some(step) => {
        if is_in_vec(step, available_steps) {
          return step.clone();
        }
        self.best_available_step(available_steps)
      }
      None => self.best_available_step(available_steps),
    }
  }

  // TODO: optimize, maybe with sorted available_steps ?
  fn best_available_step(&self, available_steps: &Vec<&Step>) -> Step {
    for step in self.favorite_steps.iter() {
      if is_in_vec(step, available_steps) {
        return step.clone();
      }
    }
    // Should never get here
    available_steps[0].clone()
  }
}

pub fn build_dnas(parents: Vec<(Score, Path)>) -> Vec<DNA> {
  let len = parents.len();
  parents
    .into_iter()
    .map(|(score, path)| DNA::new(path, score, len))
    .collect()
}
