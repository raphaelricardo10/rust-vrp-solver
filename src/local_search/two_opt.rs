use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceMatrix,
    stop_swapper::{neighborhood::Neighborhood, StopSwapper},
};

pub struct TwoOptSearcher {
    stop_swapper: StopSwapper,
}

impl TwoOptSearcher {
    pub fn new(stops: Vec<Stop>, distances: &DistanceMatrix) -> Self {
        Self {
            stop_swapper: StopSwapper::new(stops, distances),
        }
    }

    fn should_swap_stops(swap_cost: f32) -> bool {
        swap_cost < 0.0
    }

    fn find_improvements(
        &self,
        stops: &Vec<Stop>,
        neighborhood: &Neighborhood,
    ) -> Option<(usize, f32)> {
        let (swap_candidate_index, swap_cost) =
            self.stop_swapper.get_minimum_swap_cost(neighborhood, stops);

        let swap_candidate = Neighborhood::from((
            stops.as_slice(),
            swap_candidate_index,
            &self.stop_swapper.distance_service,
        ));

        if Self::should_swap_stops(swap_cost) {
            return Some((swap_candidate.current.index, swap_cost));
        }

        None
    }

    pub fn run(&self, route: &mut Route) {
        for stop_index in 1..route.stops.len() - 1 {
            let neighborhood = Neighborhood::from((
                route.stops.as_slice(),
                stop_index,
                &self.stop_swapper.distance_service,
            ));

            let (swap_candidate_index, distance_change) =
                match self.find_improvements(&route.stops, &neighborhood) {
                    Some(candidate_index) => candidate_index,
                    None => continue,
                };

            route.swap_stops(stop_index, swap_candidate_index, distance_change);
        }
    }
}
