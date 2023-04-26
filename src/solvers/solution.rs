pub trait Solution {
    type Data;
    type Cost: PartialOrd;

    fn get_cost(&self) -> Self::Cost;
    fn get_data(&self) -> &Self::Data;

    fn is_better_than(&self, other: &Self) -> bool {
        self.get_cost() < other.get_cost()
    }
}
