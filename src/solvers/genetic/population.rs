use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(crate) individuals: Vec<Individual>,
}

impl Population {
    #[allow(dead_code)]
    pub(super) fn new(individuals: Vec<Individual>) -> Self {

        Self { individuals }
    }

    pub(super) fn get_k_bests(&self, k: usize) -> &[Individual] {
        &self.individuals[..k]
    }
}
