pub mod file_reader;
pub mod brute_force_tsp;
pub mod city;
mod genetic_way;
mod genetic_simulation;

fn main() {
    //let cities : Vec<city::City> = city::City::random_cities(21, 0, 120);
    let read_file = file_reader::read_file("C:\\users\\public\\ber52.txt");
    let cities  = city::City::string_to_city(read_file);
    let mut sim = genetic_simulation::Simulation::new(
        120,
        0.9,
        0.04,
        50,
        cities
    );
    sim.run(1);
}

