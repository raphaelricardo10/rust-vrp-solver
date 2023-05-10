use std::collections::BTreeMap;

use crate::solvers::solution::Solution;

#[derive(Default, Clone)]
pub(super) struct TestSolution {
    cost: u32,
    data: BTreeMap<u32, Vec<u32>>,
}

impl Solution for TestSolution {
    type Cost = u32;
    type Data = BTreeMap<u32, Vec<u32>>;

    fn get_cost(&self) -> Self::Cost {
        self.cost
    }

    fn get_data(&self) -> &Self::Data {
        &self.data
    }
}

impl TestSolution {
    pub(super) fn insert(&mut self, sequence_id: u32, candidate_id: u32, cost: u32) {
        self.data
            .entry(sequence_id)
            .or_insert_with(Vec::new)
            .push(candidate_id);
        self.cost += cost;
    }
}
