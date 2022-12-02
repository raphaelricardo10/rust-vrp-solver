use rstest::rstest;

use super::fixtures::{population_factory, PopulationFactory};

#[rstest]
fn test_generate_random_population(mut population_factory: PopulationFactory) {
    let population = population_factory(2, 2);

    assert_ne!(population.individuals[0].chromosomes[0].stops.len(), 0);
    assert_ne!(population.individuals[0].chromosomes[0].stops.len(), 0);
}
