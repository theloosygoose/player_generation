use rnglib::{RNG, Language};
use rand::Rng;

pub struct People {
    id: Vec<usize>,
    names: Vec<String>,
    ages: Vec<u32>,
    offensive_rating: Vec<u32>,
    defensive_rating: Vec<u32>,
}

#[allow(dead_code)]
impl People {
    pub fn new() -> People {
        People { 
            id:vec![0],
            names: vec![String::from("Gusti")], 
            ages: vec![21], 
            offensive_rating: vec![100], 
            defensive_rating: vec![100]
        }
    }

    pub fn add_person(&mut self, name: String, age: u32, offensive_rating: u32, defensive_rating: u32) {
        self.id.push(self.id.len() + 1);
        self.names.push(name);
        self.ages.push(age);
        self.offensive_rating.push(offensive_rating);
        self.defensive_rating.push(defensive_rating);
    }

    pub fn say_hello(&self, index: usize) {
        println!("DOD EXAMPLE:: Hello, my name is {} and I am {} years old", self.names[index], self.ages[index]);
        println!("My Offense is {} and my Defense is {}", self.offensive_rating[index], self.defensive_rating[index]);
    }

    pub fn increase_ratings(&mut self){
        self.offensive_rating.iter_mut().for_each(|x| *x +=1);
        self.defensive_rating.iter_mut().for_each(|x| *x +=1);
    }

    pub fn add_random_player(&mut self){
        let rng = RNG::new(&Language::Roman).unwrap();

        self.id.push(self.id.len() + 1);
        self.names.push(rng.generate_name());
        self.ages.push(rand::thread_rng().gen_range(20..=40));
        self.offensive_rating.push(rand::thread_rng().gen_range(1..=100));
        self.defensive_rating.push(rand::thread_rng().gen_range(1..=100));
    }
}
