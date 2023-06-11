// destructuring

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

struct Person2 {
    name: String,
    height: u8,
}

impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person {name, height, ..} = input;

        Self {
            name,
            height,
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: String::from("Papa Doc"),
        real_name: String::from("Dr. Strange"),
        height: 185,
        happiness: false,
    };


    let Person { name: a, real_name: b, height: c, happiness: d } = papa_doc; // destructuring

    println!("They call him {} but his real name {} , he is {} cm tall and he is {}",
             a, b, c, d
    );

    let person = Person {
        name: String::from("John"),
        real_name: String::from("Johnny"),
        height: 180,
        happiness: true,
    };

    let person2 = Person2::from_person(person);

    println!("{} is {} cm tall", person2.name, person2.height);
}
