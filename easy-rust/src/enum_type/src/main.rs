use std::io;
use std::os::unix::raw::time_t;

// enum = or (enumeration)
#[derive(Debug)]
enum ThingsInTheSky {
    Sun,
    // index = 0
    Moon, // index = 1
}

#[derive(Debug)]
enum Mood {
    Happy,
    Sleepy,
    Angry,
    NotBad,
}

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn create_all_season() -> Vec<Season> {
    use Season::*;
    vec![Spring, Summer, Autumn, Winter]
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    match mood {
        Happy => 10,
        Sleepy => 5,
        Angry => 0,
        NotBad => 7,
    }
}

fn create_sky_days(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Moon,
    }
}

fn create_sky_state(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("It's sunny"),
        ThingsInTheSky::Moon => println!("It's dark"),
    }
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}

fn main() {
    let time = 12;
    let sky_state = create_sky_days(time);
    create_sky_state(&sky_state);

    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("My mood is {:?} and my happiness level is {}", my_mood, happiness_level);

    let all_season = create_all_season();
    for season in all_season {
        println!("season as u32 = {:?}" ,season as u32);
    }

    use Star::*;
    let stars = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in stars {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star"),
            size if size > 80 => println!("Biggest star {} ",size),
            _ => println!("Unknown star")
        }
    }

    let my_number_vec = vec![get_number(-800), get_number(0)];
    for number in my_number_vec {
        match number {
            Number::U32(number) => println!("u32 number = {}", number),
            Number::I32(number) => println!("i32 number = {}", number),
        }
    }
}
