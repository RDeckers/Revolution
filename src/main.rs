extern crate revolution;
extern crate rand;
use std::fmt;
use std::cmp::*;
use revolution::creature::*;
use revolution::population::*;


struct Robot{
  score: i64,
  state_response: [u8; 3]
}

impl Robot{
  fn get_gene(&self, index: usize) -> u8{
    self.state_response[index]
  }
}

impl PartialEq for Robot{
  fn eq(&self, other: &Self) -> bool{
    self.score.eq(&other.score)
  }
  fn ne(&self, other: &Self) -> bool{
    self.score.ne(&other.score)
  }
}
impl Eq for Robot{}
impl PartialOrd for Robot{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
    self.score.partial_cmp(&other.score)
  }
}
impl Ord for Robot{
  fn cmp(&self, other: &Self) -> Ordering{
    self.score.cmp(&other.score)
  }
}

impl Default for Robot{
  fn default() -> Self{
    Robot{
      score: 0,
      state_response: [0u8; 3]
    }
  }
}

impl IsCreature for Robot{
  fn mutate(&mut self){
    //let position = rand::random::<u8>() as usize;
    for position in 0..self.state_response.len(){
      self.state_response[position] = rand::random::<u8>();
    }
  }
  fn compute_fitness(&mut self) -> i64{
    let mut fitness = 0;
    for i in 0..self.state_response.len(){
      fitness += self.state_response[i] as i64;
    }
    self.score = fitness;
    fitness
  }
  fn make_child(mom: Self, dad: Self) -> Self{
    let mut new_response : [u8; 3] = unsafe{std::mem::uninitialized()};;
    for i in 0..new_response.len(){
      new_response[i] = {
        if rand::random::<u8>() & 1 == 0{
          mom.get_gene(i)
        }
        else{
          dad.get_gene(i)
        }
      }

    }
    Robot{
      score: 0,
      state_response: new_response,
    }
  }
}

impl fmt::Debug for Robot{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<{}>[", self.score).unwrap();
    for i in 0..self.state_response.len()-1{
      write!(f, "{}, ", self.state_response[i]).unwrap();
    }
    write!(f, "{}]", self.state_response[self.state_response.len()-1])
  }
}

fn main(){
  let mut population = Population::<Robot>::new(4);
  {
    let vec = population.get_vec();
    println!("{:?}", vec);
  }
  population.mutate();
  {
    let vec = population.get_vec();
    println!("{:?}", vec);
  }
  population.compute_fitness();
  population.sort();
  {
    let vec = population.get_vec();
    println!("{:?}", vec);
  }
}
