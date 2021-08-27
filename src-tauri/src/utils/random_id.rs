use rand::Rng;
use std::collections::HashMap;

pub struct RandomList<T: std::cmp::Eq + std::hash::Hash> {
  pub distribution: HashMap<T, i64>,
}
impl<T: std::cmp::Eq + std::hash::Hash> RandomList<T> {
  pub fn add(&mut self, item: T, frequency: i64) {
    self.distribution.insert(item, frequency);
  }

  /**
   * Get a random member from items you just added
   */
  pub fn random_get(&self) -> Option<&T> {
    let mut total_frequency = 0;
    for (item, frequency) in &self.distribution {
      total_frequency += frequency;
    }
    let mut rng = rand::thread_rng();
    let mut random_index = rng.gen_range(0..total_frequency);
    let mut last_item: Option<&T> = None;
    for (item, frequency) in &self.distribution {
      total_frequency -= frequency;
      if random_index >= total_frequency {
        return Some(item);
      }
      last_item = Some(item);
    }
    last_item
  }
}
impl<T: std::cmp::Eq + std::hash::Hash> Default for RandomList<T> {
  fn default() -> RandomList<T> {
    RandomList { distribution: HashMap::new() }
  }
}
