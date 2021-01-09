pub mod file_reader;
pub mod brute_force_tsp;
pub mod city;
mod genetic_way;
mod genetic_simulation;

fn main() {
    //let read_file = file_reader::read_file("C:\\users\\public\\tsp250.txt");
    //let read_file = file_reader::read_file("C:\\users\\public\\tsp500.txt");
    let read_file = file_reader::read_file("C:\\users\\public\\tsp1000.txt");
    //let read_file = file_reader::read_file("C:\\users\\public\\ber52.txt");
    //let read_file = file_reader::read_file("C:\\users\\public\\ber127.txt");
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

