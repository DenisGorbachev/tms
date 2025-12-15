#[macro_export]
macro_rules! subtype_vec_impl_all {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident<T>(Vec<T>)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        $crate::subtype_vec_def!(
            $(#[$meta])*
            $vis struct $name<T>(Vec<T>)
            $(where [$($where_clause)*])?;
        );
        $crate::subtype_vec_impl_self!($name);
        $crate::subtype_vec_impl_extend!($name);
        $crate::subtype_vec_impl_into_iter_own!($name);
        $crate::subtype_vec_impl_into_iter_ref!($name);
    };
}

#[macro_export]
macro_rules! subtype_vec_def {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident<T>(Vec<T>)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        #[derive(new, derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
        #[repr(transparent)]
        $(#[$meta])*
        $vis struct $name<T>(Vec<T>)
        $(where $($where_clause)*)?;
    };
}

#[macro_export]
macro_rules! subtype_vec_impl_self {
    ($name:ident) => {
        impl<T> $name<T> {
            /// Use this function instead of [`Default::default`] because `Default::default` requires `A: Default`, which might not be implemented
            pub fn empty() -> Self {
                Self(vec![])
            }

            pub fn push(&mut self, value: impl Into<T>) {
                self.0.push(value.into())
            }

            pub fn extend_from<F: Into<T>>(&mut self, iter: impl IntoIterator<Item = F>) {
                self.extend(iter.into_iter().map(F::into))
            }
        }
    };
}

#[macro_export]
macro_rules! subtype_vec_impl_extend {
    ($name:ident) => {
        impl<T> Extend<T> for $name<T> {
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                self.0.extend(iter);
            }
        }
    };
}

#[macro_export]
macro_rules! subtype_vec_impl_into_iter_own {
    ($name:ident) => {
        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! subtype_vec_impl_into_iter_ref {
    ($name:ident) => {
        impl<'a, T> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
    };
}

#[macro_export]
macro_rules! subtype_vec_impl_from {
    ($name:ident, $value:ty, [$($value_source:ty),+]) => {
        $(
            $crate::subtype_vec_impl_from!($name, $value, $value_source);
        )+
    };
    ($name:ident, $value:ty, $value_source:ty) => {
        impl From<$value_source> for $name<$value> {
            fn from(source: $value_source) -> Self {
                Self(vec![source.into()])
            }
        }
    };
}
