#[repr(C)]
pub struct FFIGeneticSolverParameters {
    pub population_size: u32,
    pub elite_size: usize,
    pub mutation_rate: f32,
    pub local_search_rate: f32,
    pub max_crossover_tries: u8,
    pub max_generations: u32,
}
