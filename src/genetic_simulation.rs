use rand::{thread_rng, Rng};
use super::*;

pub struct Simulation {

    iterations: usize,

    crossover_probability: f64,
    mutation_probability: f64,
    population_size: usize,

    number_of_cities: usize,
    cities: Vec<city::City>,

    number_of_mutations: usize,
    number_of_crossovers: usize,

    pub fitness: f64,
    pub dna: Vec<usize>,
}

impl Simulation {

    pub fn new(iterations: usize,
               crossover_probability: f64,
               mutation_probability: f64,
               population_size: usize,
               cities: Vec<city::City>) -> Self {

        assert_eq!(population_size % 10, 0, "population_size:{} should be divisible by 10", population_size);

        let number_of_cities = cities.len();
        let number_of_mutations = 0;
        let number_of_crossovers = 0;
        let fitness = 0.0;
        let dna: Vec<usize> = Vec::new();

        Simulation {
            iterations,
            crossover_probability,
            mutation_probability,
            population_size,
            number_of_cities,
            cities,
            number_of_mutations,
            number_of_crossovers,
            fitness,
            dna,
        }
    }

    fn generate_children(&mut self, mom: &genetic_way::Individual, dad: &genetic_way::Individual) -> (genetic_way::Individual, genetic_way::Individual) {
        if thread_rng().gen_bool(self.crossover_probability) {
            self.number_of_crossovers += 2;
            mom.crossover_pmx(dad, &self.cities)
        } else {
            (mom.clone(), dad.clone())
        }
    }

    fn might_mutate_child(&mut self, child: &mut genetic_way::Individual) {
        if thread_rng().gen_bool(self.mutation_probability) {
            child.mutate(&self.cities);
            self.number_of_mutations += 1;
        }
    }

    pub fn generate_population(&mut self, individuals: Vec<genetic_way::Individual>) -> Vec<genetic_way::Individual> {
        assert_eq!(self.population_size % 2, 0, "population_size:{} should be divisible by 2", self.population_size);

        let cumulative_weights = get_cumulative_weights(&individuals);
        let mut next_population = Vec::new();

        for _ in 0..(self.population_size / 2 ) { // generate two individuals per iteration

            let (mom, dad) = select_parents(&cumulative_weights, &individuals);
            let (mut daughter, mut son) = self.generate_children(&mom, &dad);
            self.might_mutate_child(&mut daughter);
            self.might_mutate_child(&mut son);

            next_population.push(daughter);
            next_population.push(son);
        }
        next_population
    }

    pub fn run(&mut self, skip: usize) {
        assert!(skip > 0, "skip must be 1 or larger");

        let mut population = random_population(self.population_size, &self.cities);
        let mut champion = find_fittest(&population);
        let mut first_champ = champion.clone();
        first_champ.dna.push(0);
        println!("\n --------------- \n STATS AT START \n --------------- \n");
        println!("Path at start: {:?}", first_champ.dna);
        println!("Fitness at start: {} ", first_champ.fitness);
        println!("Path length at start: {} ", genetic_way::path_calculator(&champion.dna, &self.cities));


        for _ in 0..self.iterations {

            let challenger = find_fittest(&population);
            population = self.generate_population(population);

            if champion.fitness <= challenger.fitness {
                champion = challenger;
            }
        }

        self.fitness = champion.fitness;
        self.dna = champion.dna;
        self.dna.push(0);

        self.print();
    }

    pub fn print(&self) {

        let x = self.population_size * self.iterations;

        println!("\n --------------- \n STATS \n --------------- \n");
        println!("BEST TRAVEL PATH: {:?}", self.dna);
        println!("Fitness Score: {} ", self.fitness);
        println!("Path Score: {} ", genetic_way::path_calculator(&self.dna, &self.cities));
        println!("{} mutations out of {} individuals produced", self.number_of_mutations, x);
        println!("{} cross-overs out of {} individuals produced", self.number_of_crossovers, x);

        println!("\n --------------- \n SPECS \n --------------- \n");
        println!("iterations: {:?}", self.iterations);
        println!("crossover_probability: {:?}", self.crossover_probability);
        println!("mutation_probability: {:?}", self.mutation_probability);
        println!("population_size: {:?}", self.population_size);
        println!("number_of_cities: {:?}", self.number_of_cities);
        println!("\n --------------- \n END \n --------------- \n");

    }
}

pub fn random_dna(n: usize) -> Vec<usize> {
    let mut v:Vec<usize> = (1..n).collect();
    thread_rng().shuffle(&mut v);
    v.insert(0,0);
    v
}

pub fn select_parents<'a>(w: &[f64], individuals: &'a [genetic_way::Individual]) -> (&'a genetic_way::Individual, &'a genetic_way::Individual) {
    let mom_index = select_index(w);
    let dad_index = select_index(w);
    (&individuals[mom_index], &individuals[dad_index])
}

pub fn find_fittest(population: &[genetic_way::Individual]) -> genetic_way::Individual {

    let mut best_individual = &population[0];

    for individual in population {
        if best_individual.fitness > individual.fitness {
            best_individual = individual;
        }
    }
    best_individual.clone()
}

pub fn get_cumulative_weights(individuals: &[genetic_way::Individual]) -> Vec<f64> {

    let mut running_sum = 0.0;
    let mut cumulative_weights = vec![running_sum];

    for i in individuals {
        running_sum += i.fitness;
        cumulative_weights.push(running_sum);
    }
    cumulative_weights
}

pub fn random_population(population_size: usize, cities: &[city::City]) -> Vec<genetic_way::Individual> {

    let number_of_cities = cities.len();
    let mut individuals:Vec<genetic_way::Individual> = Vec::new();

    for _ in 0..population_size {
        let dna = random_dna(number_of_cities);
        let indiv = genetic_way::Individual::new(dna, &cities);
        individuals.push(indiv);
    }
    individuals
}

pub fn select_index(cumulative_weights: &[f64]) -> usize {
    let w_sum = cumulative_weights.last().unwrap();
    let r: f64 = thread_rng().gen_range(0.0, *w_sum);
    cumulative_weights.iter().rposition(|&w| w < r).unwrap()
}