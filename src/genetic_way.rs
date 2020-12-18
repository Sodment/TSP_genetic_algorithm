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


    pub fn crossover_pmx(&self, other: &Individual, cities: &[city::City]) -> (Self, Self){

        let n = self.dna.len();
        let mut rng = thread_rng();
        let start = rng.gen_range(0, n - 1);
        let end = rng.gen_range(start + 1, n);

        let child1_dna = crossover_dna_pmx(&self.dna, &other.dna, start, end);
        let child2_dna = crossover_dna_pmx(&other.dna, &self.dna, start, end);

        let child1 = Individual::new(child1_dna, cities);
        let child2 = Individual::new(child2_dna, cities);

        (child1, child2)
    }

    pub fn mutate(&mut self, cities: &[city::City]) {
        let i = thread_rng().gen_range(0, self.dna.len() - 1);
        self.dna.swap(i, i + 1);
        self.fitness = fitness_calculator(&self.dna, &cities);
    }


}

pub fn crossover_dna_pmx(parent1: &[usize], parent2: &[usize], start: usize, end: usize) -> Vec<usize>{
    let parent1_slice = &parent1[start..=end];
    let mut child: Vec<usize> = vec![];

    for i in 0..parent2.len(){
        if !parent1_slice.contains(&parent2[i]){
            child.push(parent2[i]);
        }
    }
    let end_slice = &child.split_off(start);
    child.extend_from_slice(parent1_slice);
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
    let length = cities.len() - 1;
    let mut d = 0.0;
    for i in 0..length {
        let (j, k) = (dna[i], dna[i + 1]);
        d += cities[j].distance_to(&cities[k]);
    }
    d
}