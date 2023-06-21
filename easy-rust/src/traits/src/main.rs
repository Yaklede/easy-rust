// traits // verbs / adjective
// it's very similar interface


extern crate core;

struct Animal {
    name: String
}


trait Canine {
    fn bark(&self) {
        println!("default method bark")
    }

    fn run(&self) {
        println!("default method run");
    }
}

impl Canine for Animal {
    fn bark(&self) {
        println!("{} mang mang", self.name)
    }
    fn run(&self) {
        println!("{} run mang",self.name)
    }
}

use std::fmt::{Display, Formatter};
use std::ops::Range;

struct Cat {
    name: String,
    age: u8
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

struct Monster {
    health: i32
}

struct Wizard {}

struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(" You strike with your sword! Your opponent's health is now  {}", opponent.health)
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 5;
        println!(" You strike with your hand! Your opponent's health is now  {}", opponent.health)
    }
}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!("YOu attack with your bow! Your opponent's health is {}", opponent.health);
        }
    }
    fn attach_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You attack with a rock your opponent's health {}", opponent.health);
        }
    }
}

impl FightFromDistance for Ranger {}
impl FightClose for Wizard {}

fn main() {
    // let animal = Animal {
    //     name: "my_dog".to_string()
    // };
    // animal.run();
    // animal.bark();
    // let my_cat = Cat {
    //     name: "cat".to_string(),
    //     age: 10
    // };
    // println!("{my_cat}")
    let radagast = Wizard {};
    let aragorn = Ranger {};
    let mut monster = Monster {
        health: 50
    };
    radagast.attack_with_sword(&mut monster);
    aragorn.attack_with_bow(&mut monster, 5);
    println!("{}", monster.health);
}