pub trait Actor<T> {
    fn handle(&mut self, cmd: T);
}