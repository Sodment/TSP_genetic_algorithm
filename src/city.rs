use rand::{thread_rng, Rng};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct City{
    pub id: usize,
    pub x: i64,
    pub y: i64,
}

impl City
{
    pub fn new(id: usize, x: i64, y: i64) -> Self{
        City{id, x, y}
    }

    pub fn distance_to(&self, other: &City ) -> f64{
        let result: f64 = (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt();
        return result;
    }

    pub fn string_to_city(contents: String) -> Vec<City>{
        let mut cities: Vec<City> = Vec::new();
        for (i, line) in contents.lines().enumerate().skip(1){
            let values: Vec<i64> = line.split(" ")
                    .map(|val| i64::from_str(val.trim())
                    .unwrap())
                    .collect();
            let c = City::new(i, values[1], values[2]);
            cities.push(c);
        }
        cities
    }

    pub fn random_cities(n: usize, mn: i64, mx: i64) -> Vec<City>{
        let mut rng = thread_rng();
        let mut cities: Vec<City> = Vec::new();
        println!("{}", n);

        for i in 1..n+1
        {
            let x: i64 = rng.gen_range(mn, mx);
            let y: i64 = rng.gen_range(mn, mx);
            let c = City::new(i, x, y);
            println!("{} {} {}", i, x, y);
            cities.push(c);
        }
        cities
    }


}