use super::creature::*;

pub struct Population<C>{
  population : Vec<Box<C>>, //Linked list is no good. Need fast RA for selecting random samples.
  //So either use a vector of pointers or used indirect indices.
}

impl<C> Population<C> where C: IsCreature + Default + Ord{
  pub fn new(population_size: usize) -> Self{
    Population{
      population: {
        let mut vec = Vec::<Box<C>>::new();
        for _ in 0..population_size{
          vec.push(Box::new(C::default()));
        }
        vec
      }
    }
  }
  pub fn compute_fitness(&mut self){
    self.population.iter_mut().map(|c| c.compute_fitness()).collect::<Vec<_>>();
  }
  pub fn mutate(&mut self){
    self.population.iter_mut().map(|c| c.mutate()).collect::<Vec<_>>();
  }
  pub fn sort(&mut self){
    self.population.sort_by(|a,b| (a).cmp(&b));
  }
  pub fn get_vec(&self) -> &Vec<Box<C>>{
    &self.population
  }
}
