pub trait IteratorRemainderExtension {
    type Item;

    fn to_remainder(self, div: u64) -> RemainderIterator<Self> where Self: Sized, Self: Iterator<Item=u64>;
}

impl<I> IteratorRemainderExtension for I where I: Iterator<Item=u64>, I: Sized {
    type Item = u64;

    fn to_remainder(self, div: u64) -> RemainderIterator<Self> where Self: Iterator<Item=u64> {
        RemainderIterator { iter: self, div: div }
    }
}

pub struct RemainderIterator<I> where I: Iterator<Item=u64> {
    iter: I,
    div: u64,
}

impl<I> Iterator for RemainderIterator<I> where I: Iterator<Item=u64> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.iter.next() {
            None => None,
            Some(x) => Some(x % self.div),
        }
    }
}
