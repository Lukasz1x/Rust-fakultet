#[derive(Debug)]
pub struct Node<T>
{
    value: T,
    previous: Option<Box<Node<T>>>
}

impl<T> Node<T>
{
    pub fn new(value: T, previous:Option<Box<Node<T>>>) -> Box<Self>
    {
        Box::new(Self{
            value,
            previous,
        })
    }

    pub fn previous(self) -> Option<Box<Node<T>>>
    {
        self.previous
    }

    pub fn value(&self) -> &T
    {
        &self.value
    }
}