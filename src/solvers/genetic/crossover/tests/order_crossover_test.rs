use crate::domain::stop::Stop;
use crate::solvers::genetic::crossover::offspring::Offspring;
use crate::solvers::genetic::crossover::order_crossover::OrderCrossover;
use crate::solvers::genetic::individual::Individual;
use crate::solvers::genetic::tests::fixtures::{individual_factory, parent_slice_factory};
use crate::solvers::genetic::tests::fixtures::{IndividualFactory, ParentSliceFactory};
use crate::tests::fixtures::routes_fixture::{route_factory, RouteFactory};
use crate::tests::fixtures::services_fixture::distance_service;
use crate::tests::fixtures::stops_fixture::stops;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::services::distance::distance_service::DistanceService;

#[rstest]
fn test_slice_cost_is_correct(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) {
    let individual = individual_factory(1);
    let route = &individual.chromosomes[0];
    let slice_cost = Individual::calculate_slice_cost(&route.stops, &distance_service);

    assert_eq!(slice_cost, route.total_distance());
}

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
fn test_can_drop_gene_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(2);

    let chromosome_without_duplicates =
        Individual::drop_gene_duplicates(&parent.chromosomes[0], &slice);

    assert_eq!(chromosome_without_duplicates.len(), 3);

    for gene in chromosome_without_duplicates {
        assert!(!slice.contains(&gene));
    }
}

#[rstest]
fn test_can_drop_all_genes_from_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(3);

    let chromosome_without_duplicates =
        Individual::drop_gene_duplicates(&parent.chromosomes[0], &slice);

    assert_eq!(chromosome_without_duplicates.len(), 2);
    assert_eq!(chromosome_without_duplicates[0].id, 0);
    assert_eq!(chromosome_without_duplicates[1].id, 0);
}

#[rstest]
fn test_can_generate_offspring_chromosome(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
    mut parent_slice_factory: ParentSliceFactory,
) {
    let (_, parent1_slice) = parent_slice_factory(2);
    let parent2 = individual_factory(1);

    let chromosome = Individual::make_offspring_chromosome(
        &parent1_slice,
        parent2.chromosomes[0].clone(),
        &distance_service,
    );

    assert_eq!(chromosome.stops.len(), 3);
}

#[rstest]
fn test_can_generate_offspring_chromosome_dropping_a_whole_chromosome(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
    mut parent_slice_factory: ParentSliceFactory,
) {
    let (_, parent1_slice) = parent_slice_factory(3);
    let parent2 = individual_factory(1);

    let chromosome = Individual::make_offspring_chromosome(
        &parent1_slice,
        parent2.chromosomes[0].clone(),
        &distance_service,
    );

    assert_eq!(chromosome.stops.len(), 1);
}

#[rstest]
fn test_can_insert_parent_slice_in_empty_offspring(
    stops: Vec<Stop>,
    route_factory: RouteFactory,
    distance_service: DistanceService,
) {
    let chromosome = route_factory(stops[0..=0].to_vec());
    let insertion_point = (0, 0);

    let mut offspring = Individual::new(vec![chromosome]);

    let slice = &stops[1..=3];
    let slice_cost = Individual::calculate_slice_cost(slice, &distance_service);

    offspring
        .insert_parent_slice(
            insertion_point,
            slice.to_vec(),
            slice_cost,
            &distance_service,
        )
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
