extern crate revolution;
extern crate rand;
use rand::Rng;
use std::fmt;
use std::cmp::*;
use revolution::creature::*;
use revolution::population::*;
/*v<>^P = 5 actions
W.T = 3 space posibilities |--> 3^5 = 243 genes without reduction.
v^<>X = 5 viewspaces      / */

#[derive(Copy, Clone, PartialEq, Eq)]
enum Space{
  Empty =0,
  Wall =1,
  Trash =2,
}

#[derive(Clone, Copy)]
enum Action{//TODO: Move enums into a module instead, define a "range of" variable instead of an actions array.
  MoveUp,
  MoveDown,
  MoveLeft,
  MoveRight,
  PickUp,
}

enum Score{//TODO: enum cannot contain same value twice.
  Wall = -1,//TODO: meta-evolve these values.
  PickUp = 25,
  Miss = -10
}

impl fmt::Debug for Action{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "{}", self)
  }
}

impl fmt::Display for Action{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "{}", self)
  }
}

const ACTIONS : [Action; 5] = [Action::MoveUp, Action::MoveDown, Action::MoveLeft, Action::MoveRight, Action::PickUp];//TODO: we shouldn't need to do this.
const N_GENES : usize = 243;
const FIELD_X : usize = 10;
const FIELD_Y : usize = 10;
const THRASH_COUNT : usize = FIELD_X*FIELD_Y/2;

struct Robot{
  score: i64,
  state_response: [Action; N_GENES],
  field: [[Space; FIELD_X+2];FIELD_Y+2]
}

impl Robot{
  fn get_gene(&self, index: usize) -> Action{
    self.state_response[index]
  }
  fn generate_field(&mut self){
    self.field[0] = [Space::Wall; FIELD_X+2]; //top wall
    for i in 1..FIELD_Y+1{
      for j in 1..FIELD_X+1{
        self.field[i][j] = Space::Empty; //center
      }
      //self.field[i][0] = self.field[i][FIELD_X+1] = Space::Wall; //left and right wall
      self.field[i][FIELD_X+1] = Space::Wall;
      self.field[i][0] = Space::Wall;
    }
    self.field[FIELD_Y+1] = [Space::Wall; FIELD_X+2];//bottom wall
    let mut rng = rand::thread_rng();
    let mut thrash_count = 0;
    while thrash_count != THRASH_COUNT{
      let x = rng.gen::<usize>() % FIELD_X;
      let y = rng.gen::<usize>() % FIELD_Y;
      if self.field[y][x] == Space::Empty{
        self.field[y][x] = Space::Trash;
        thrash_count += 1;
      }
    }
  }
  fn get_response(&self, x: usize, y : usize) -> Action{
    let mut index = self.field[y][x] as usize;
    index = index*3+self.field[y+1][x] as usize;
    index = index*3+self.field[y-1][x] as usize;
    index = index*3+self.field[y][x+1] as usize;
    index = index*3+self.field[y][x-1] as usize;
    self.get_gene(index)
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
    let mut rng = rand::thread_rng();
    Robot{
      field: [[Space::Empty; FIELD_X+2];FIELD_Y+2],
      score: 0,
      state_response: [*rng.choose(&ACTIONS).unwrap(); N_GENES]
    }
  }
}

impl IsCreature<i64> for Robot{
  fn mutate(&mut self){
    //let position = rand::random::<u8>() as usize;
    let mut rng = rand::thread_rng();
    while rng.gen_weighted_bool(2) {
      let index = rng.next_u64() as usize % self.state_response.len();
      self.state_response[index] = *rng.choose(&ACTIONS).unwrap();
    }
    //println!("Mutated {} times!", n_mutations);
  }
  fn get_fitness(&self) -> i64{
    self.score
  }
  fn compute_fitness(&mut self, _runs: usize) -> i64{
    self.generate_field();
    let mut fitness : i64 = 0;
    let (mut x, mut y) = (1,1);
    for _ in 0..(THRASH_COUNT+FIELD_X*FIELD_Y){
      match self.get_response(x, y){ //TODO: DRY
        Action::PickUp =>
         if self.field[y][x] == Space::Trash {
          fitness += Score::PickUp as i64;
          self.field[y][x] = Space::Empty;
         }
         else{
          fitness += Score::Miss as i64;
         },
        Action::MoveDown =>
         if self.field[y+1][x] == Space::Wall {
          fitness += Score::Wall as i64;
         }
         else{
           y += 1;
         },
        Action::MoveUp =>
         if self.field[y-1][x] == Space::Wall {
          fitness += Score::Wall as i64;
         }
         else{
           y -= 1;
         },
        Action::MoveLeft =>
         if self.field[y][x-1] == Space::Wall {
          fitness += Score::Wall as i64;
         }
         else{
           x -= 1;
         },
        Action::MoveRight =>
         if self.field[y][x+1] == Space::Wall {
          fitness += Score::Wall as i64;
         }
         else{
           x += 1;
         },
      }
      //fitness += self.state_response[i] as i64;
    }
    self.score = fitness;
    fitness
  }
  fn make_child(mom: &Self, dad: &Self) -> Self{
    let mut rng = rand::thread_rng();
    let mut new_response : [Action; N_GENES] = unsafe{std::mem::uninitialized()};
    for i in 0..new_response.len(){
      //new_response[i] = cmp::max(mom.get_gene(i), dad.get_gene(i));
      new_response[i] = {
        if rng.gen(){
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
      field : [[Space::Empty; FIELD_X+2];FIELD_Y+2]
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
  let mut population = Population::<Robot, i64>::new(100);
  //println!("Computing...");
  population.compute_fitness(1);
  population.assign_weights_ranked();
  for _ in 0..1000{
    //println!("Mutating...");
    population.mutate();
    //println!("Computing...");
    population.compute_fitness(1);
    //println!("Sorting...");
    population.sort();
    //println!("Breeding...");
    //println!("{:?}", population.get_weights());
    println!("{}", population.get_vec().last().unwrap());
    population.breed_next_generation();
  }
  //println!("Computing...");
  population.compute_fitness(1);
  population.sort();
  println!("{:?}", population.get_weights());
  println!("{}", population);
}
