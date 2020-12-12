//mod city;
pub mod file_reader;
pub mod brute_force_tsp;
pub mod city;
mod genetic_way;

fn main() {
    //let cities : Vec<city::City> = city::City::random_cities(21, 0, 120);
    let read_file = file_reader::read_file("C:\\users\\public\\ber52.txt");
    let cities  = city::City::string_to_city(read_file);
    println!("{}", brute_force_tsp::greedy_way(&cities));
}

