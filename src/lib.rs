#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(extended_key_value_attributes)]
#![deny(missing_docs)]
#![warn(clippy::pedantic, clippy::cargo, clippy::unwrap_used)]
#![doc = include_str!("../README.markdown")]

mod private {
    /// A type that is guaranteed to not exist outside this crate. We cannot use primitive tuple
    /// pairs because the [`DifferentType`] auto trait will consequently not get implemented for
    /// things like `((T1, T1), (T2, T2))`.
    pub struct TypePair<A, B>(A, B);

    /// This is located in a private module in order to prevent client code from being able to name
    /// this unsealed trait, so that no other negative implementations can be possible. We can't
    /// use a `Sealed` trait because auto traits don't currently allow additional trait bounds. We
    /// also can't directly define the [`NotSameTypeAs<T>`] trait using auto traits because auto
    /// traits currently don't allow generic parameters. Auto traits also behave subtly different
    /// from the kind of "blanket" implementation we'd expect (especially with compound types), so
    /// we limit to only our private named tuple type [`TypePair`].
    pub auto trait DifferentTypes {}
    impl<T> !DifferentTypes for TypePair<T, T> {}

    /// This is the sealed version of [`super::NotSameTypeAs`] to prevent downstream crates from
    /// implementing this trait on equal types.
    pub trait NotSameTypeAs<T> {}
    impl<T1, T2> NotSameTypeAs<T1> for T2 where TypePair<T1, T2>: DifferentTypes {}

    /// This is the sealed version of [`super::SameTypeAs`] to prevent downstream crates from
    /// implementing this trait on different types.
    pub trait SameTypeAs<T> {}
    impl<T> SameTypeAs<T> for T {}

    #[cfg(test)]
    mod test {
        use super::DifferentTypes;
        use static_assertions::assert_impl_all;

        #[test]
        fn different_types_trait_should_work_with_nested_pairs() {
            assert_impl_all!(((i32, i32), (f64, f64)): DifferentTypes);
        }
    }
}

/// [`NotSameTypeAs<T>`] is a marker trait that is automatically implemented for all types that do
/// not alias to the same type `T`. Lifetimes are not considered.
///
/// # Examples
///
/// ```
/// use spidermeme::NotSameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// assert_impl_all!(i32: NotSameTypeAs<i64>);
/// assert_not_impl_any!(i32: NotSameTypeAs<i32>);
/// ```
///
/// Different types with identical structures aren't equal:
///
/// ```
/// use spidermeme::NotSameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// struct A(i32);
/// struct B(i32);
///
/// assert_impl_all!(A: NotSameTypeAs<B>);
/// assert_not_impl_any!(A: NotSameTypeAs<A>);
/// ```
///
/// Type aliases should work as expected:
///
/// ```
/// use spidermeme::NotSameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// type AliasOfI32 = i32;
///
/// assert_impl_all!(AliasOfI32: NotSameTypeAs<i64>);
/// assert_not_impl_any!(AliasOfI32: NotSameTypeAs<i32>);
/// ```
///
/// Generics should work too:
///
/// ```
/// use spidermeme::NotSameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
/// use std::marker::PhantomData;
///
/// struct Generic<T>(PhantomData<T>);
///
/// assert_impl_all!(Generic<i32>: NotSameTypeAs<Generic<f64>>);
/// assert_not_impl_any!(Generic<i32>: NotSameTypeAs<Generic<i32>>);
/// ```
///
/// Different kinds of references should work too:
///
/// ```
/// use spidermeme::NotSameTypeAs;
///
/// struct Pair<T1, T2>(T1, T2);
///
/// trait Homogeneous {
///     fn is_same(&self) -> bool;
/// }
/// impl<T1, T2> Homogeneous for Pair<T1, T2> {
///     fn is_same(&self) -> bool { true }
/// }
/// impl<T1, T2> Pair<T1, T2> where T1: NotSameTypeAs<T2> {
///     fn is_same(&self) -> bool { false }
/// }
///
/// let x: i32 = 1;
/// let mut y: i32 = 1;
/// let same = Pair(&x, &x);
/// let different = Pair(&x, &mut y);
///
/// assert!(same.is_same());
/// assert!(!different.is_same());
/// ```
///
/// Different lifetimes don't make them different. The following two examples do not compile:
///
/// ```compile_fail
/// use spidermeme::NotSameTypeAs;
///
/// struct Generic<'a, 'b>(&'a(), &'b()) where &'a(): NotSameTypeAs<&'b()>;
///
/// let x = ();
/// let y = ();
/// let z = Generic(&x, &y);
/// ```
///
/// ```compile_fail
/// use spidermeme::NotSameTypeAs;
///
/// struct Generic<'a, 'b>(&'a str, &'b str) where &'a str: NotSameTypeAs<&'b str>;
///
/// let x: &'static str = "x";
/// let y = String::from("y");
/// let z = Generic(x, &y);
/// ```
///
/// Function pointers should work:
///
/// ```
/// use spidermeme::NotSameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// assert_impl_all!(fn(i32) -> i32: NotSameTypeAs<fn(i32) -> i64>);
/// assert_not_impl_any!(fn(i32) -> i32: NotSameTypeAs<fn(i32) -> i32>);
/// ```
///
/// Closures should also work:
///
/// ```
/// use spidermeme::NotSameTypeAs;
///
/// struct Pair<T1, T2>(T1, T2);
///
/// trait Homogeneous {
///     fn is_same(&self) -> bool;
/// }
/// impl<T1, T2> Homogeneous for Pair<T1, T2> {
///     fn is_same(&self) -> bool { true }
/// }
/// impl<T1, T2> Pair<T1, T2> where T1: NotSameTypeAs<T2> {
///     fn is_same(&self) -> bool { false }
/// }
///
/// let x = || 1;
/// let y = || 1;
/// let same = Pair(x, x);
/// let different = Pair(x, y);
///
/// assert!(same.is_same());
/// assert!(!different.is_same());
/// ```
pub trait NotSameTypeAs<T>: private::NotSameTypeAs<T> {}
impl<T1, T2> NotSameTypeAs<T1> for T2 where private::TypePair<T1, T2>: private::DifferentTypes {}

/// [`SameTypeAs<T>`] is a marker trait that is automatically implemented for all types that alias
/// to the same type `T`. Lifetimes are not considered.
///
/// # Examples
///
/// ```
/// use spidermeme::SameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// assert_impl_all!(i32: SameTypeAs<i32>);
/// assert_not_impl_any!(i32: SameTypeAs<f64>);
/// ```
///
/// Different types with identical structures aren't equal:
///
/// ```
/// use spidermeme::SameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// struct A(i32);
/// struct B(i32);
///
/// assert_impl_all!(A: SameTypeAs<A>);
/// assert_not_impl_any!(A: SameTypeAs<B>);
/// ```
///
/// Type aliases should work as expected:
///
/// ```
/// use spidermeme::SameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// type AliasOfI32 = i32;
///
/// assert_impl_all!(AliasOfI32: SameTypeAs<i32>);
/// assert_not_impl_any!(AliasOfI32: SameTypeAs<i64>);
/// ```
///
/// Generics should work too:
///
/// ```
/// use spidermeme::SameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
/// use std::marker::PhantomData;
///
/// struct Generic<T>(PhantomData<T>);
///
/// assert_impl_all!(Generic<i32>: SameTypeAs<Generic<i32>>);
/// assert_not_impl_any!(Generic<i32>: SameTypeAs<Generic<f64>>);
/// ```
///
/// Different kinds of references should work too:
///
/// ```
/// use spidermeme::SameTypeAs;
///
/// struct Pair<T1, T2>(T1, T2);
///
/// trait Heterogeneous {
///     fn is_same(&self) -> bool;
/// }
/// impl<T1, T2> Heterogeneous for Pair<T1, T2> {
///     fn is_same(&self) -> bool { false }
/// }
/// impl<T1, T2> Pair<T1, T2> where T1: SameTypeAs<T2> {
///     fn is_same(&self) -> bool { true }
/// }
///
/// let x: i32 = 1;
/// let mut y: i32 = 1;
/// let same = Pair(&x, &x);
/// let different = Pair(&x, &mut y);
///
/// assert!(same.is_same());
/// assert!(!different.is_same());
/// ```
///
/// Different lifetimes don't make them different. The following two examples do compile:
///
/// ```
/// use spidermeme::SameTypeAs;
///
/// struct Generic<'a, 'b>(&'a(), &'b()) where &'a(): SameTypeAs<&'b()>;
///
/// let x = ();
/// let y = ();
/// let z = Generic(&x, &y);
/// ```
///
/// ```
/// use spidermeme::SameTypeAs;
///
/// struct Generic<'a, 'b>(&'a str, &'b str) where &'a str: SameTypeAs<&'b str>;
///
/// let x: &'static str = "x";
/// let y = String::from("y");
/// let z = Generic(x, &y);
/// ```
///
/// Function pointers should work:
///
/// ```
/// use spidermeme::SameTypeAs;
/// use static_assertions::{assert_impl_all, assert_not_impl_any};
///
/// assert_impl_all!(fn(i32) -> i32: SameTypeAs<fn(i32) -> i32>);
/// assert_not_impl_any!(fn(i32) -> i32: SameTypeAs<fn(i32) -> i64>);
/// ```
///
/// Closures should also work:
///
/// ```
/// use spidermeme::SameTypeAs;
///
/// struct Pair<T1, T2>(T1, T2);
///
/// trait Heterogeneous {
///     fn is_same(&self) -> bool;
/// }
/// impl<T1, T2> Heterogeneous for Pair<T1, T2> {
///     fn is_same(&self) -> bool { false }
/// }
/// impl<T1, T2> Pair<T1, T2> where T1: SameTypeAs<T2> {
///     fn is_same(&self) -> bool { true }
/// }
///
/// let x = || 1;
/// let y = || 1;
/// let same = Pair(x, x);
/// let different = Pair(x, y);
///
/// assert!(same.is_same());
/// assert!(!different.is_same());
/// ```
pub trait SameTypeAs<T>: private::SameTypeAs<T> {}
impl<T> SameTypeAs<T> for T {}
