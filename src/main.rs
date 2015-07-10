extern crate revolution;
extern crate rand;
use rand::Rng;
use std::fmt;
use std::cmp;
use std::cmp::*;
use revolution::creature::*;
use revolution::population::*;

struct Robot{
  score: u64,
  state_response: [u8; 256]
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
      state_response: {
        let mut new_response = [0u8; 256];
        let mut rng = rand::thread_rng();
        for i in 0..new_response.len(){
          new_response[i] = (rng.next_u32() % 256) as u8;
        }
        new_response
      }
    }
  }
}

impl IsCreature<u64> for Robot{
  fn mutate(&mut self){
    //let position = rand::random::<u8>() as usize;
    let mut rng = rand::thread_rng();
    while rng.next_u32() & 1 == 0{
      let index = rng.next_u64() as usize % self.state_response.len();
      self.state_response[index] = rand::random::<u8>();
    }
    //println!("Mutated {} times!", n_mutations);
  }
  fn get_fitness(&self) -> u64{
    self.score
  }
  fn compute_fitness(&mut self, _runs: usize) -> u64{
    let mut fitness = 0;
    for i in 0..self.state_response.len(){
      fitness += self.state_response[i] as u64;
    }
    self.score = fitness;
    fitness
  }
  fn make_child(mom: &Self, dad: &Self) -> Self{
    let mut new_response : [u8; 256] = unsafe{std::mem::uninitialized()};;
    for i in 0..new_response.len(){
      new_response[i] = cmp::max(mom.get_gene(i), dad.get_gene(i));/*{
        if rand::random::<u8>() & 1 == 0{
          mom.get_gene(i)
        }
        else{
          dad.get_gene(i)
        }
      }*/
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

impl fmt::Display for Robot{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "{}", self.score)
  }
}

fn main(){
  let mut population = Population::<Robot>::new(1000);
  //println!("Computing...");
  population.compute_fitness(1);
  for _ in 0..100{
    //println!("Mutating...");
    population.mutate();
    //println!("Computing...");
    population.compute_fitness(1);
    //println!("Sorting...");
    population.sort();
    //println!("Breeding...");
    population.breed_next_generation();
  }
  //println!("Computing...");
  population.compute_fitness(1);
  population.sort();
  println!("{}", population);
}
