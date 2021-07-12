use std::ops::Index;

pub trait Deserializer<D>: Index<String>
where
    D: Deserializer<D>,
{
    fn get_field(&self, key: &str) -> Option<&D>;
    fn get_type(&self) -> Option<String>;
    fn as_map(&self) -> Option<&D>;
    fn as_set(&self) -> Option<Vec<&D>>;
    fn as_str(&self) -> Option<&str>;
}

impl<D> Index<String> for &dyn Deserializer<D, Output = D>
where
    D: Deserializer<D> + Index<String>,
    <D as Index<String>>::Output: Deserializer<D> + Index<String>,
{
    type Output = D;

    fn index(&self, index: String) -> &Self::Output {
        self.get_field(index.as_ref()).unwrap()
    }
}

/*
impl<D, S> Index<S> for &dyn Deserializer<D, S, Output = D>
where
    D: Deserializer<D, S> + Index<S>,
    <D as Index<S>>::Output: Deserializer<D, S> + Index<S>,
    S: Into<String>,
{
    type Output = D;

    fn index(&self, index: S) -> &Self::Output {
        self.get_field(&index.into()).unwrap()
    }
}*/
