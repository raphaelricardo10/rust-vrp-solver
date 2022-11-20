use rstest::rstest;

use crate::{solvers::genetic::{genetic_solver::GeneticSolver, individual::Individual}, domain::stop::Stop};

use super::fixtures::{route_service_factory, RouteServiceFactory, route_factory, RouteFactory, stops};

#[rstest]
fn test_generate_random_individual(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(2);

    let individual = GeneticSolver::generate_random_individual(&mut route_service);

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

    individual.swap_genes(address1, address2, -2.0);

    assert_eq!(individual.get_gene(address1).unwrap().id, stops[2].id);
    assert_eq!(individual.get_gene(address2).unwrap().id, stops[1].id);
    assert_eq!(individual.fitness, 7.0);
}