#[derive(Debug)]
pub struct Node<T>
{
    value: T,
    previous: Option<*mut Node<T>>
}

impl<T> Node<T>
{
    pub fn new(value: T, previous:Option<*mut Node<T>>) -> *mut Self
    {
        let mut a = Box::new(Self{
            value,
            previous,
        });
        Box::into_raw(a)

    }

    pub fn previous(&self) -> Option<*mut Node<T>>
    {
        self.previous
    }

    pub fn value(&self) -> &T
    {
        &self.value
    }
}