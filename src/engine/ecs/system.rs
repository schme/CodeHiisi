pub trait System {
    fn run<T>(&mut self, data: T);
}
