use rstest::fixture;

use crate::{
    services::distance::distance_service::DistanceService,
    solvers::genetic::{
        crossover::parent_slice::ParentSlice,
        individual::Individual,
        tests::fixtures::{individual_factory, IndividualFactory},
    },
    tests::fixtures::services_fixture::distance_service,
};

pub(crate) type ParentSliceFactory = Box<dyn FnMut(usize) -> (Individual, ParentSlice)>;

#[fixture]
pub(crate) fn parent_slice_factory(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) -> ParentSliceFactory {
    let wrapper = move |number_of_genes| -> (Individual, ParentSlice) {
        let parent = individual_factory(1);
        let slice = Vec::from_iter(
            parent.chromosomes[0]
                .stops
                .iter()
                .skip(1)
                .take(number_of_genes)
                .cloned(),
        );

        let parent_slice = ParentSlice::new(slice, &distance_service);

        (parent, parent_slice)
    };

    Box::new(wrapper)
}
