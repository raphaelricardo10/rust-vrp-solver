use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::path_node::PathNode;

#[derive(Copy, Clone)]
pub(crate) struct Path<'a> {
    prev: PathNode<'a>,
    current: PathNode<'a>,
    next: PathNode<'a>,
    cost: f64,
}

impl<'a> Path<'a> {
    pub(crate) fn new(
        prev: PathNode<'a>,
        current: PathNode<'a>,
        next: PathNode<'a>,
        distance_service: &DistanceService,
    ) -> Option<Path<'a>> {
        let mut path = Path {
            prev,
            current,
            next,
            cost: 0.0,
        };

        path.cost = path.calculate_cost(distance_service)?;

        Some(path)
    }

    pub(crate) fn from_window(
        window: &'a [Stop],
        base_index: usize,
        distance_service: &DistanceService,
    ) -> Option<Path<'a>> {
        let mut path = Path {
            prev: PathNode::new(base_index, &window[0]),
            current: PathNode::new(base_index + 1, &window[1]),
            next: PathNode::new(base_index + 2, &window[2]),
            cost: 0.0,
        };

        path.cost = path.calculate_cost(distance_service)?;

        Some(path)
    }

    pub(crate) fn from_stop_index(
        stops: &'a Vec<Stop>,
        stop_index: usize,
        distance_service: &DistanceService,
    ) -> Option<Path<'a>> {
        Self::new(
            PathNode::new(stop_index - 1, &stops[stop_index - 1]),
            PathNode::new(stop_index, &stops[stop_index]),
            PathNode::new(stop_index + 1, &stops[stop_index + 1]),
            distance_service,
        )
    }

    fn calculate_cost(&self, distance_service: &DistanceService) -> Option<f64> {
        Some(
            distance_service.get_distance(self.prev.get_stop(), self.current.get_stop())?
                + distance_service.get_distance(self.current.get_stop(), self.next.get_stop())?,
        )
    }

    pub(crate) fn get_prev(&self) -> &PathNode<'a> {
        &self.prev
    }

    pub(crate) fn get_current(&self) -> &PathNode<'a> {
        &self.current
    }

    pub(crate) fn get_next(&self) -> &PathNode<'a> {
        &self.next
    }

    pub(crate) fn get_cost(&self) -> f64 {
        self.cost
    }

    pub(crate) fn set_current(&mut self, path: PathNode<'a>,distance_service: &DistanceService) {
        self.current = path;
        self.cost = self.calculate_cost(distance_service).unwrap();
    }
}
