use crate::domain::{route::Route, stop::Stop};

type Gene = Stop;
type Chromosome = Route;

pub(super) struct Individual {
    fitness: f64,
    chromosomes: Vec<Chromosome>,
}

impl Individual {
    pub fn new(chromosomes: Vec<Chromosome>) -> Self {
        let fitness = Self::calculate_fitness(&chromosomes);

        Self {
            fitness,
            chromosomes,
        }
    }

    fn calculate_fitness(chromosomes: &[Chromosome]) -> f64 {
        chromosomes
            .iter()
            .map(|chromosome| chromosome.total_distance())
            .sum()
    }
}
