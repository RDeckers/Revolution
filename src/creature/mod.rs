pub trait IsCreature{
  fn mutate(&mut self);
  fn compute_fitness(&mut self) -> i64; //TODO: fitness in arbitrary type.
  fn make_child(Self, Self) -> Self; //TODO: references & lifetimes
}
