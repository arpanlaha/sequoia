#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub enum OctreeNode<T> {
    Leaf(Option<T>),
    Subtree(Box<[OctreeNode<T>; 8]>),
}

pub struct Octree<T> {
    root: OctreeNode<T>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
