#[repr(C)]
pub struct GeneticAlgorithmParameters {
    pub number_of_routes: usize,
    pub number_of_stops: usize,
    pub population_size: u32,
    pub elite_size: usize,
    pub mutation_rate: f64,
    pub max_crossover_tries: u8,
    pub max_generations: u32,
}
