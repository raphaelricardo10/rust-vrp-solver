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
pub(crate) type ParentSliceFactory = Box<dyn FnMut(usize) -> (Individual, HashSet<Stop>)>;

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

#[fixture]
pub(crate) fn parent_slice_factory(
    mut individual_factory: IndividualFactory,
) -> ParentSliceFactory {
    let wrapper = move |number_of_genes| -> (Individual, HashSet<Stop>) {
        let parent = individual_factory(1);
        let slice = HashSet::from_iter(
            parent.chromosomes[0]
                .stops
                .iter()
                .skip(1)
                .take(number_of_genes)
                .cloned(),
        );

        (parent, slice)
    };

    Box::new(wrapper)
}
