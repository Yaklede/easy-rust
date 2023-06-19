// Other collection types
// HashMap, BTreeMap
// Key, Value

use std::collections::HashMap;
use std::collections::BTreeMap;

struct  City {
    name: String,
    population: HashMap<u32, u32>
}

struct OrderCity {
    name: String,
    population: BTreeMap<u32, u32>
}

fn main() {
    more_hashmap();
}

fn more_hashmap() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for canadian_city in canadian_cities {
        city_hashmap.insert(canadian_city, "Canada");
    }

    for german_city in german_cities {
        city_hashmap.insert(german_city, "Germany");
    }
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeld111"));

    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1,"a");
    book_hashmap.insert(1,"b");

    println!("{:?}", book_hashmap.get(&1));

    if let Some(book) = book_hashmap.get(&1)  {
        println!("Already got a one {} ",book);
    } else {
        book_hashmap.insert(1, "other One");
    }
}

fn default_method_hashmap() {
    let mut tallin = City {
        name: "Tallinn".to_string(),
        population: HashMap::new()
    };

    let mut orderTallin = OrderCity {
        name: "orderTallinn".to_string(),
        population: BTreeMap::new()
    };

    tallin.population.insert(1372, 3_250);
    tallin.population.insert(1400, 5_250);
    tallin.population.insert(1500, 6_250);

    orderTallin.population.insert(1000, 3_000);
    orderTallin.population.insert(2000, 50_000);
    orderTallin.population.insert(3000, 30_000);

    for (year, population) in tallin.population {
        println!("In the year {} ", year);
        println!("In the population {}", population);
    }

    for (year, population) in orderTallin.population {
        println!("In the year {}", year);
        println!("In the population {}", population)
    }
}
