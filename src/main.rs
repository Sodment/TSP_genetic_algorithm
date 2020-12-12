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
        1000,
        0.7,
        0.04,
        100,
        cities
    );
    sim.run(1);
    /*
    let mut v: Vec<usize> = vec![];
    v = sim.dna.clone();
    v.push(0);
     */


}

