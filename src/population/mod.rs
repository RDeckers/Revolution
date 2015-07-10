use super::creature::*;
use std::ops::Add;
use std::fmt;

pub struct Population<C,T>{//TODO: use typeof or similar to remove any reference to T?
                           //     Can template at fn<T> in the impl, but this causes issues when implementing non-linear weights as
                           //     it requires us to keep track of the total of T for several runs.

                           //have creatures implement a get-fitness call, allowing to grab all the fitnesses and compute relevant weights in step.
  total_fitness: T,
  population : Vec<Box<C>>
}

impl<C,T> Population<C,T> where C: IsCreature<T>+Default + Ord, T: Add<Output = T> + Default{
  pub fn new(population_size: usize) -> Self{
    Population{
      total_fitness : T::default(),//TODO: use std::num::zero instead of Default when availible?
      population: {
        let mut vec = Vec::<Box<C>>::new();
        for _ in 0..population_size{
          vec.push(Box::new(C::default()));
        }
        vec
      }
    }
  }
  pub fn compute_fitness(&mut self, runs: usize){//TODO: use std::num::zero instead of Default when availible?
    self.total_fitness = self.population.iter_mut().fold(T::default(), |acc, c| acc+c.compute_fitness(runs));
  }
  //pub fn assign_weights_ranked(&mut self){
  //}
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

impl<C,T> fmt::Display for Population<C,T> where C: fmt::Display{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "[").unwrap();
    for i in 0..self.population.len()-1{
      write!(f, "{}, ", self.population[i]).unwrap();
    }
    write!(f, "{}]", self.population[self.population.len()-1])
  }
}

impl<C,T> fmt::Debug for Population<C,T> where C: fmt::Debug{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "[").unwrap();
    for i in 0..self.population.len()-1{
      write!(f, "{:?}, ", self.population[i]).unwrap();
    }
    write!(f, "{:?}]", self.population[self.population.len()-1])
  }
}
