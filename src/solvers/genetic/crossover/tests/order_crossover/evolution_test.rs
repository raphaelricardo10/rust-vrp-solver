use crate::services::distance::distance_service::DistanceService;
use crate::solvers::genetic::crossover::offspring::Offspring;
use crate::solvers::genetic::crossover::order_crossover::OrderCrossover;
use crate::solvers::genetic::tests::fixtures::{individual_factory, IndividualFactory};
use crate::tests::fixtures::services_fixture::distance_service;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

#[rstest]
fn test_can_generate_a_offspring(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let parent1 = individual_factory(1);
    let parent2 = individual_factory(1);

    let offspring = parent1
        .crossover_with(parent2, &mut rng, &distance_service)
        .unwrap();

    assert_ne!(offspring.fitness, 0.0);
}

#[rstest]
fn test_can_generate_offspring_better_than_parents(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) {
    let parent1 = individual_factory(1);
    let parent2 = individual_factory(1);

    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let crossover_op = OrderCrossover::new(100);

    let mut offspring = Offspring::new(parent1.clone(), parent2.clone(), crossover_op);
    offspring
        .try_to_evolve(&mut rng, &distance_service)
        .unwrap();

    assert!(offspring.individual.fitness < parent1.fitness);
    assert!(offspring.individual.fitness < parent2.fitness);
}
