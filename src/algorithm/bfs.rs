/// A parallel util for doing breadth first search
/// When traversing the graph-like datastructure
pub struct Bfs {}

pub trait One2Many<T> {
    type Many: Iterator<Item = T>;

    fn convert(&mut self) -> Self::Many;
}
