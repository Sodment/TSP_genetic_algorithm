use super::*;
pub use city::City;

pub fn greedy_way(vector: &[City], start_index: usize) -> Vec<usize>
{
    let mut city_copy:Vec<City> = vector.iter().cloned().collect();

    let mut path: Vec<usize> = vec![];
    let mut current_city: City = vector[start_index].clone();
    let mut nearest_city: City;

    for _ in 0..city_copy.len()
    {
        path.push(current_city.clone().id - 1);
        let index = city_copy.iter().position(|x| x.id == current_city.id).unwrap();
        let _useless = city_copy.remove(index);
        nearest_city = find_nearest_city(&city_copy, &current_city);
        current_city = nearest_city;

    }
    path
}

pub fn find_nearest_city(vector: &Vec<City>, current: &City) -> City
{
    let mut min_distance: f64 = f64::MAX;
    let mut nearest= city::City::new(0, 0, 0);
    for city in vector.iter()
    {
        let distance_to_city: f64 = city::City::distance_to(current, city);
        if distance_to_city <= min_distance
        {
            nearest = city.clone();
            min_distance = distance_to_city;
        }
    }
    nearest

}