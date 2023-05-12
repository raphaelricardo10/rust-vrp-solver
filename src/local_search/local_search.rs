pub trait LocalSearch<T> {
    fn run(&self, target: &mut T);
}
