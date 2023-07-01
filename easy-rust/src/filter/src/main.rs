use std::fmt::format;
use std::vec;

// filter
// filter_map
fn main() {
    let num_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 9];
    let filter = num_vec.into_iter().filter(|number| number < &5).collect::<Vec<i32>>();
    println!("{filter:?}");

    let company_vec = vec![
        Company::new("company_1", "ceo1"),
        Company::new("company_2", ""),
        Company::new("company_3", "ceo3"),
        Company::new("company_4", ""),
    ];

    let all_the_ceos = company_vec.into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<_>>();

    println!("{all_the_ceos:?}");

    more_option_function();
}

struct Company {
    name: String,
    ceo: Option<String>
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string())
        };

        Self {
            name: name.to_string(),
            ceo
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

//example ok, ok_or, ok_or_else
//ok - Result to Option
//ok_else Option to Result
//ok_or_else Option to Result with closure ||
fn more_option_function() {
    let company_vec = vec![
        Company::new("Name A", "CEO A"),
        Company::new("Name B", ""),
        Company::new("Name C", "CEO C"),
        Company::new("Name D", "")
    ];
    let mut result_vec = vec![];
    company_vec
        .iter()
        .for_each(|company| {
            result_vec.push(company.get_ceo().ok_or_else(|| {
                let error_message = format!("No Ceo found for {}", company.name);
                error_message
            }))
        });

    println!("{result_vec:?}")
}
