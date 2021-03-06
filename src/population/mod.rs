extern crate rand;

use super::creature::*;
use std::ops::Add;
use std::fmt;

pub struct Population<C,T>{//TODO: use typeof or similar to remove any reference to T?
                           //     Can template at fn<T> in the impl, but this causes issues when implementing non-linear weights as
                           //     it requires us to keep track of the total of T for several runs.

                           //have creatures implement a get-fitness call, allowing to grab all the fitnesses and compute relevant weights in step.
  total_fitness: T,
  total_weights: u64,
  weights: Vec<u64>,
  population : Vec<Box<C>>
}

impl<C, T> Population<C, T> where C: IsCreature<T>+Default + Ord, T: Default + Add<Output = T>{
  pub fn new(population_size: usize) -> Self{
    Population{
      total_weights : 0,
      total_fitness : T::default(),//TODO: use std::num::zero instead of Default when availible?
      population: {
        let mut vec = Vec::<Box<C>>::with_capacity(population_size);
        for _ in 0..population_size{
          vec.push(Box::new(C::default()));
        }
        vec
      },
      weights : vec![0u64; population_size]
    }
  }

  fn pick_parent(&self) -> &C{
    if self.total_weights == 0{
      let index = rand::random::<usize>() % self.population.len();
      return &self.population[index];
    }
    let pick = rand::random::<u64>() % self.total_weights;
    let mut sum = 0;
    let mut index = 0;
    while sum <= pick{
      sum += self.weights[index];
      index += 1;
    }
    &self.population[index-1]
  }

  pub fn breed_next_generation(&mut self){
    let mut new_population = Vec::<Box<C>>::new();
    while new_population.len() != self.population.len(){
      let mom = self.pick_parent();
      let dad = self.pick_parent();
      new_population.push(Box::new(C::make_child(mom, dad)));
    }
    self.total_fitness = T::default(); //TODO: use zero
    self.population = new_population;
  }
  pub fn compute_fitness(&mut self, runs: usize){//TODO: use std::num::zero instead of Default when availible?
    self.total_fitness = self.population.iter_mut().fold(T::default(), |acc, c| acc+c.compute_fitness(runs));
  }
  pub fn assign_weights_ranked(&mut self){
    for i in 0..self.population.len(){
      self.weights[i] =  i as u64 +1
    }
    self.total_weights = (self.population.len()*(self.population.len()+1)/2) as u64
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
  pub fn get_weights(&self) -> &Vec<u64>{
    &self.weights
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
