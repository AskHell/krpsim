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

impl DNA {
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
    for step in self.favorite_steps {
      if is_in_vec(&step, available_steps) {
        return step.clone();
      }
    }
    // Should never get here
    available_steps[0].clone()
  }
}

pub fn build_dnas(parents: Vec<(Score, Path)>) -> Vec<DNA> {
  unimplemented!()
}
