use std::ops::Add;
pub trait IsCreature<T : Add>{
  fn mutate(&mut self);
  fn compute_fitness(&mut self, runs: usize) -> T; //TODO: fitness in arbitrary type.
  fn make_child(Self, Self) -> Self; //TODO: references & lifetimes
}
