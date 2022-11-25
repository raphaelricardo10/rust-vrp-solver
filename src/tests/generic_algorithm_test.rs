use std::collections::HashSet;

use rand::thread_rng;
use rstest::rstest;

use crate::{
    domain::stop::Stop,
    services::distance::distance_service::DistanceService,
    solvers::genetic::{genetic_solver::GeneticSolver, individual::Individual},
};

use super::fixtures::{
    distance_service, route_factory, route_service_factory, stops, RouteFactory,
    RouteServiceFactory,
};

#[rstest]
fn test_generate_random_individual(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(2);

    let individual = GeneticSolver::generate_random_individual(&mut route_service);

    for chromosome in individual.chromosomes.iter() {
        assert_eq!(chromosome.stops.first().unwrap().id, 0);
        assert_eq!(chromosome.stops.last().unwrap().id, 0);
    }

    assert_ne!(individual.chromosomes[0].stops.len(), 0);
    assert_ne!(individual.chromosomes[1].stops.len(), 0);
}

#[rstest]
fn test_generate_random_population(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(2);

    let population = GeneticSolver::generate_random_population(3, &mut route_service);

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
    route_service_factory: RouteServiceFactory,
    distance_service: DistanceService,
) {
    let mut route_service = route_service_factory(1);

    let individual = GeneticSolver::generate_random_individual(&mut route_service);

    let route = &individual.chromosomes[0];

    let slice_cost = GeneticSolver::calculate_slice_cost(&route.stops, &distance_service);

    assert_eq!(slice_cost, route.total_distance());
}

#[rstest]
fn test_can_generate_a_offspring(
    route_service_factory: RouteServiceFactory,
    distance_service: DistanceService,
) {
    let mut route_service = route_service_factory(1);

    let population = GeneticSolver::generate_random_population(2, &mut route_service);

    let parent1 = population.individuals[0].clone();
    let parent2 = population.individuals[1].clone();

    let mut rng = thread_rng();

    let offspring = GeneticSolver::make_offspring(
        parent1.clone(),
        parent2.clone(),
        &mut rng,
        &distance_service,
    )
    .unwrap();

    assert_ne!(offspring.fitness, 0.0);
}

#[rstest]
fn test_can_drop_gene_duplicates(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    let individual = GeneticSolver::generate_random_individual(&mut route_service);

    let mut gene_set: HashSet<Stop> = HashSet::new();
    gene_set.insert(individual.chromosomes[0].stops[1]);
    gene_set.insert(individual.chromosomes[0].stops[2]);

    let chromosome_without_duplicates =
        GeneticSolver::drop_gene_duplicates(&individual.chromosomes[0], &gene_set);

    assert_eq!(chromosome_without_duplicates.len(), 3);

    for gene in chromosome_without_duplicates {
        assert!(!gene_set.contains(gene));
    }
}

#[rstest]
fn test_can_drop_all_genes_from_duplicates(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    let individual = GeneticSolver::generate_random_individual(&mut route_service);

    let mut gene_set: HashSet<Stop> = HashSet::new();
    gene_set.insert(individual.chromosomes[0].stops[1]);
    gene_set.insert(individual.chromosomes[0].stops[2]);
    gene_set.insert(individual.chromosomes[0].stops[3]);

    let chromosome_without_duplicates =
        GeneticSolver::drop_gene_duplicates(&individual.chromosomes[0], &gene_set);

    assert_eq!(chromosome_without_duplicates.len(), 2);
    assert_eq!(chromosome_without_duplicates[0].id, 0);
    assert_eq!(chromosome_without_duplicates[1].id, 0);
}

#[rstest]
fn test_can_generate_offspring_chromosome(
    route_service_factory: RouteServiceFactory,
    distance_service: DistanceService,
) {
    let mut route_service = route_service_factory(1);

    let population = GeneticSolver::generate_random_population(2, &mut route_service);

    let parent1 = population.individuals[0].clone();
    let parent2 = population.individuals[1].clone();

    let mut parent1_slice: HashSet<Stop> = HashSet::new();
    parent1_slice.insert(parent1.chromosomes[0].stops[1]);
    parent1_slice.insert(parent1.chromosomes[0].stops[2]);

    let chromosome = GeneticSolver::make_offspring_chromosome(
        &parent1_slice,
        parent2.chromosomes[0].clone(),
        &distance_service,
    );

    assert_eq!(chromosome.stops.len(), 3);

    for gene in chromosome.stops {
        assert!(!parent1_slice.contains(&gene));
    }
}

#[rstest]
fn test_can_generate_offspring_chromosome_dropping_a_whole_chromosome(
    route_service_factory: RouteServiceFactory,
    distance_service: DistanceService,
) {
    let mut route_service = route_service_factory(1);

    let population = GeneticSolver::generate_random_population(2, &mut route_service);

    let parent1 = population.individuals[0].clone();
    let parent2 = population.individuals[1].clone();

    let mut parent1_slice: HashSet<Stop> = HashSet::new();
    parent1_slice.insert(parent1.chromosomes[0].stops[1]);
    parent1_slice.insert(parent1.chromosomes[0].stops[2]);
    parent1_slice.insert(parent1.chromosomes[0].stops[3]);

    let chromosome = GeneticSolver::make_offspring_chromosome(
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

    let slice = vec![stops[1], stops[2], stops[3]];
    let slice_cost = GeneticSolver::calculate_slice_cost(&slice, &distance_service);

    GeneticSolver::insert_parent_slice_in_offspring(
        &mut offspring,
        insertion_point,
        slice,
        slice_cost,
        &distance_service,
    )
    .unwrap();

    assert_ne!(offspring.fitness, 0.0);
}
