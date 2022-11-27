use rand::{thread_rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::{
    domain::stop::Stop,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
    solvers::genetic::{
        crossover::order_crossover::OrderCrossover, genetic_solver::GeneticSolver,
        individual::Individual,
    },
    tests::fixtures::{IndividualFactory, ParentSliceFactory},
};

use super::fixtures::{
    distance_service, distances, individual_factory, parent_slice_factory, population_factory,
    route_factory, route_service_factory, stops,
};
use super::fixtures::{PopulationFactory, RouteFactory, RouteServiceFactory};

#[rstest]
fn test_generate_random_individual(mut individual_factory: IndividualFactory) {
    let individual = individual_factory(2);

    for chromosome in individual.chromosomes.iter() {
        assert_eq!(chromosome.stops.first().unwrap().id, 0);
        assert_eq!(chromosome.stops.last().unwrap().id, 0);
    }

    assert_ne!(individual.chromosomes[0].stops.len(), 0);
    assert_ne!(individual.chromosomes[1].stops.len(), 0);
}

#[rstest]
fn test_generate_random_population(mut population_factory: PopulationFactory) {
    let population = population_factory(2, 2);

    assert_ne!(population.individuals[0].chromosomes[0].stops.len(), 0);
    assert_ne!(population.individuals[0].chromosomes[0].stops.len(), 0);
}

#[rstest]
fn test_fitness_is_correct(stops: Vec<Stop>, route_factory: RouteFactory) {
    let route1 = route_factory(stops[0..=1].to_vec());
    let route2 = route_factory(stops[2..=3].to_vec());

    let individual = Individual::new(vec![route1, route2]);

    assert_eq!(individual.fitness, 4.0);
}

#[rstest]
fn test_gene_swap(stops: Vec<Stop>, route_factory: RouteFactory) {
    let route = route_factory(stops[..4].to_vec());

    let mut individual = Individual::new(vec![route]);

    let address1 = (0, 1);
    let address2 = (0, 2);

    individual.swap_genes(address1, address2, -2.0).unwrap();

    assert_eq!(individual.fitness, 7.0);
    assert_eq!(individual.get_gene(address1).unwrap().id, stops[2].id);
    assert_eq!(individual.get_gene(address2).unwrap().id, stops[1].id);
}

#[rstest]
fn test_slice_cost_is_correct(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) {
    let individual = individual_factory(1);
    let route = &individual.chromosomes[0];
    let slice_cost = OrderCrossover::calculate_slice_cost(&route.stops, &distance_service);

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

    let offspring =
        OrderCrossover::make_offspring(parent1, parent2, &mut rng, &distance_service).unwrap();

    assert_ne!(offspring.fitness, 0.0);
}

#[rstest]
fn test_can_drop_gene_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(2);

    let chromosome_without_duplicates =
        OrderCrossover::drop_gene_duplicates(&parent.chromosomes[0], &slice);

    assert_eq!(chromosome_without_duplicates.len(), 3);

    for gene in chromosome_without_duplicates {
        assert!(!slice.contains(&gene));
    }
}

#[rstest]
fn test_can_drop_all_genes_from_duplicates(mut parent_slice_factory: ParentSliceFactory) {
    let (parent, slice) = parent_slice_factory(3);

    let chromosome_without_duplicates =
        OrderCrossover::drop_gene_duplicates(&parent.chromosomes[0], &slice);

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

    let chromosome = OrderCrossover::make_offspring_chromosome(
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

    let chromosome = OrderCrossover::make_offspring_chromosome(
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
    let slice_cost = OrderCrossover::calculate_slice_cost(slice, &distance_service);

    OrderCrossover::insert_parent_slice_in_offspring(
        &mut offspring,
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
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    route_service_factory: RouteServiceFactory,
    mut individual_factory: IndividualFactory,
) {
    let parent1 = individual_factory(1);
    let parent2 = individual_factory(1);

    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let route_service = route_service_factory(2);
    let mut solver = GeneticSolver::new(
        stops,
        &distances,
        10,
        3,
        0.05,
        10,
        100,
        route_service,
        &mut rng,
    );

    let offspring = match solver.make_better_offspring(parent1.clone(), parent2.clone()) {
        Some(offspring) => offspring,
        None => panic!("Could not generate better offspring"),
    };

    assert!(offspring.fitness < parent1.fitness);
    assert!(offspring.fitness < parent2.fitness);
}

#[rstest]
fn test_genetic_algorithm_can_optimize_route(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    route_service_factory: RouteServiceFactory,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let route_service = route_service_factory(2);
    let mut solver = GeneticSolver::new(
        stops,
        &distances,
        10,
        3,
        0.05,
        10,
        100,
        route_service,
        &mut rng,
    );
    solver.solve();

    let solution_v1 = solver.solution.result.get(&0).unwrap();
    let solution_v2 = solver.solution.result.get(&1).unwrap();

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
