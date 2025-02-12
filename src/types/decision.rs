use subtype::subtype;

subtype!(
    #[derive(Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Copy, Debug)]
    pub struct Decision[T](Option<T>) where [T: Clone];
);

impl<T: Clone> Decision<T> {}
