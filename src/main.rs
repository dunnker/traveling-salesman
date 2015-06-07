extern crate rand;
extern crate num;

mod city;
mod tour;
mod population;
mod ga;

use city::*;
use population::*;
use ga::*;

fn main() {

    let mut city_list: Vec<City> = Vec::new();
    city_list.push(City { x: 60, y: 200 });
    city_list.push(City { x: 180, y: 200 });
    city_list.push(City { x: 80, y: 180 });
    city_list.push(City { x: 140, y: 180 });
    city_list.push(City { x: 20, y: 160 });
    city_list.push(City { x: 100, y: 160 });
    city_list.push(City { x: 200, y: 160 });
    city_list.push(City { x: 140, y: 140 });
    city_list.push(City { x: 40, y: 120 });
    city_list.push(City { x: 100, y: 120 });
    city_list.push(City { x: 180, y: 100 });
    city_list.push(City { x: 60, y: 80 });
    city_list.push(City { x: 120, y: 80 });
    city_list.push(City { x: 180, y: 60 });
    city_list.push(City { x: 20, y: 40 });
    city_list.push(City { x: 100, y: 40 });
    city_list.push(City { x: 200, y: 40 });
    city_list.push(City { x: 20, y: 20 });
    city_list.push(City { x: 60, y: 20 });
    city_list.push(City { x: 160, y: 20 });

    let rng = &mut rand::thread_rng();
    let mut population = Population::new();
    population.initialize_from_cities(rng, &city_list);

    // fittest_tour borrows temporarily from population:
    {
        let fittest_tour = population.get_fittest();
        println!("starting fittest {}", fittest_tour.get_distance());
    }

    for _ in 1..1000 {
        population = GA::evolve_population(rng, population);
    }

    let fittest_tour = population.get_fittest();
    println!("ending fittest {}", fittest_tour.get_distance());
}
