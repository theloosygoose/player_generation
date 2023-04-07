// DOD
struct People {
    names: Vec<String>,
    ages: Vec<u32>,
    offensive_rating: Vec<u32>,
    defensive_rating: Vec<u32>
}

impl People {
    fn manual_new() -> People {
        People { 
            names: vec![String::from("Gusti")], 
            ages: vec![21], 
            offensive_rating: vec![100], 
            defensive_rating: vec![100]
        }
    }

    fn add_person(&mut self, name: String, age: u32, offensive_rating: u32, defensive_rating: u32) {
        self.names.push(name);
        self.ages.push(age);
        self.offensive_rating.push(offensive_rating);
        self.defensive_rating.push(defensive_rating);
    }

    fn say_hello(&self, index: usize) {
        println!("DOD EXAMPLE:: Hello, my name is {} and I am {} years old", self.names[index], self.ages[index]);
    }
}



fn main() {
    let mut league_players = People::manual_new();

    league_players.add_person(String::from("Joel Embiid"), 29, 97, 99);

    league_players.say_hello(1);

    league_players.say_hello(0);

}
