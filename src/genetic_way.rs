use super::city;
use rand::{thread_rng, Rng};

pub const MIN_POSITIVE: f64 = 2.2250738585072014e-308f64;

#[derive(Debug, Clone)]
pub struct Individual {
    pub dna: Vec<usize>,
    pub fitness: f64,
}

impl Individual {

    pub fn new(dna: Vec<usize>, cities: &[city::City]) -> Self {
        let fitness = fitness_calculator(&dna, &cities);
        Individual { dna, fitness }
    }


    pub fn cross_over(&self, other: &Individual, cities: &[city::City]) -> (Self, Self) {

        let n = self.dna.len();
        let mut rng = thread_rng();
        let start = rng.gen_range(0, n - 1);
        let end = rng.gen_range(start + 1, n);

        let daughter_dna = crossover_dna(&self.dna, &other.dna, start, end);
        let son_dna = crossover_dna(&other.dna, &self.dna, start, end);

        let daughter = Individual::new(daughter_dna, cities);
        let son = Individual::new(son_dna, cities);

        (daughter, son)
    }

    pub fn mutate(&mut self, cities: &[city::City]) {
        let i = thread_rng().gen_range(0, self.dna.len() - 2);
        let j = thread_rng().gen_range(i + 1, self.dna.len() - 1);
        /*
        println!("i: {}, j: {}", i ,j);
        println!("ORIGINAL");
        for i in self.dna.clone(){
            print!("{} --> ", self.dna[i]);
        }
        println!();
        */
        let mut slice = &mut self.dna.clone()[i..=j];
        slice.reverse();
        self.dna.splice(i..=j, slice.iter().cloned());
        /*
        println!("AFTER MUTATION");
        for i in self.dna.clone(){
            print!("{} --> ", self.dna[i]);
        }
        println!();
         */
        self.fitness = fitness_calculator(&self.dna, &cities);
    }


}

fn crossover_dna(mom: &[usize], dad: &[usize], start: usize, end: usize) -> Vec<usize> {

    let mom_slice = &mom[start..=end];
    let mut child: Vec<usize> = Vec::new();

    for i in 0..dad.len() {
        if !mom_slice.contains(&dad[i]) {
            child.push(dad[i]);
        }
    }

    let end_slice = &child.split_off(start);
    child.extend_from_slice(mom_slice);
    child.extend_from_slice(end_slice);
    child
}

pub fn fitness_calculator(dna: &[usize], cities: &[city::City]) -> f64
{
    let d = dna.windows(2)
        .fold(MIN_POSITIVE, |acc, w| acc + cities[w[0]].distance_to(&cities[w[1]]));
    1.0 / d

}

pub fn path_calculator(dna: &[usize], cities: &[city::City]) -> f64
{
    let d = dna.windows(2)
        .fold(MIN_POSITIVE, |acc, w| acc + cities[w[0]].distance_to(&cities[w[1]]));
    d
}