use std::collections::HashSet;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::fixture;

use crate::{
    domain::stop::Stop,
    solvers::genetic::{individual::Individual, population::Population},
};

use crate::tests::fixtures::services_fixture::{route_service_factory, RouteServiceFactory};

pub(crate) type IndividualFactory = Box<dyn FnMut(u32) -> Individual>;
pub(crate) type PopulationFactory = Box<dyn FnMut(u32, u32) -> Population>;

#[fixture]
pub(crate) fn individual_factory(route_service_factory: RouteServiceFactory) -> IndividualFactory {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let wrapper = move |number_of_chromosomes| -> Individual {
        let mut route_service = route_service_factory(number_of_chromosomes);

        Individual::from_random(&mut rng, &mut route_service)
    };

    Box::new(wrapper)
}

#[fixture]
pub(crate) fn population_factory(route_service_factory: RouteServiceFactory) -> PopulationFactory {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let wrapper = move |number_of_individuals, number_of_chromosomes| -> Population {
        let mut route_service = route_service_factory(number_of_chromosomes);

        Population::from_random(number_of_individuals, &mut rng, &mut route_service)
    };

    Box::new(wrapper)
}