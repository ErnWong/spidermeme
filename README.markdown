| WARNING: This crate currently depends on nightly rust unstable features. |
|---|

<div align="center">
  <h1>spidermeme</h1>
  <p><strong>Rust traits to check for type equality and type inequality.<br>Useful as a building block for more complicated compile-time constraints.</strong></p>
  <p>
    <a href="https://crates.io/crates/spidermeme"><img alt="crates.io" src="https://meritbadge.herokuapp.com/spidermeme"></a>
    <a href="https://docs.rs/spidermeme"><img alt="docs.rs" src="https://docs.rs/spidermeme/badge.svg"></a>
    <a href="https://github.com/ErnWong/spidermeme/actions/workflows/ci.yml"><img alt="ci" src="https://github.com/ErnWong/spidermeme/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://codecov.io/github/ErnWong/spidermeme?branch=master"><img alt="Coverage" src="https://codecov.io/github/ErnWong/spidermeme/coverage.svg?branch=master"></a>
  </p>
  <hr>
<pre>
 ______________________________________________________________________________
|_________________________ _______      |    |                              ___|
| ______________   ___    |  ___  |     |    |             ___           .-` | |
||                /   \   | |   | |     |    |            /   \         | _..| |
||                | o o   | |   | |     |    |            o   |         ||  || |
||  N  Y  P  D    \   / __  |___| |     |    |         __ \   / __      ||  || |
||             ..-'   ''  \    o--|     |    |        |  |______  \     ||  || |
||____________ |   \______\_____  |     |    |       .'  |      \  \    ||  || |
|_____________ |---__________/    |     |  _______.-' _.-/      \  |    ||  || |
|              \         | \____  |     |    \_____.-' <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">\        /</a>\ |    ||_ || |
|               \       / \____|  |     |               <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">\       /</a>| | ___|  ''| |
|                \     /          |     |    |           <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">\      /</a>__/ --..'--.| |
|       _____     =====   |_______|     |    |            ======\   ..   `-----|
|      /     \   <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/     \</a>  |____________ |    |           <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/       \</a>  | ''| |   _|
|     /  ___  \  <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/  _   \</a> |             |____|_________ <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/         |</a> |   | |  | |
|    |  |   |   <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/  / \   \</a>                             <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/    //   /</a>  |   | |  | |
|___ |  |___|  <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/  / __ \  \</a>                           <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/   / /   /</a> | '-. | |  | |
|     \       <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/_/</a>        <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">\_\</a>                         <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">/  /'  /  /</a>   `-. '' |  |_|
|      \____ / /          \ \                       <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">|  /    \  \</a>      `-. |    |
|            //           \ \__                     <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">\__\     \__\</a>        `-----|
|           /|             \___\                     \  |     \ \              |
|          /_|                                       \ /      \ /              |
|____________________________________________________\_/______|_|______________|

</pre>
  <hr>
</div>

# Provided traits

## `spidermeme::SameTypeAs<T>`

An automatically implemented marker trait to check if two types are equal.

## `spidermeme::NotSameTypeAs<T>`

An automatically implemented marker trait to check if two types aren't equal.

# Examples

```rust
use spidermeme::{SameTypeAs, NotSameTypeAs};

struct MyPair<T1, T2>(T1, T2);

trait ProcessPair {
    fn process(&self);
}

impl<T1, T2> ProcessPair for MyPair<T1, T2> {
    fn process(&self) {
        println!("Pair of two different types.");
    }
}

impl<T1, T2> MyPair<T1, T2>
where
    T1: SameTypeAs<T2>,
{
    fn process(&self) {
        println!("Pair of same type.");
    }
}

struct UniquePair<T1, T2>
where
    T1: NotSameTypeAs<T2>,
{
    a: T1,
    b: T2,
}

impl<T1, T2> UniquePair<T1, T2>
where
    T1: NotSameTypeAs<T2>,
{
    pub fn new(a: T1, b: T2) -> Self {
        Self { a, b }
    }
}

fn main() {
    println!("Hello");
    // Prints "Pair of same type."
    MyPair(1_i32, 2_i32).process();

    // Prints "Pair of two different types."
    MyPair(1_i32, 2_i16).process();

    // Valid.
    let x = UniquePair::<i32, f64>::new(1, 2.0);

    // The following fails to compile:
    // let y = UniquePair::<i32, i32>::new(1, 2);
}
```

# How type equality works

Type equality is pretty straightforward. The [`SameTypeAs`] trait has a blanket implementation using the same generic parameter. The basic principle looks like this when simplified:

```rust
impl<T> Same<T> for T {}
```

This was inspired by numerous comments floating around on the web.

# How type inequality works

Type inequality uses [`negative_impls`](doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html) and [`auto_traits`](doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html). A naive implementation would be like the following:

```rust
pub auto trait DifferentNaive {}
impl<T> !DifferentNaive for (T, T) {}
```

However, this will give false positives, as the auto trait will not be implemented for types that contain `(T, T)`. For example, the naive implementation will fail the following:

```rust,compile_fail
use static_assertions::assert_impl_all;
assert_impl_all!(((i32, i32), (f64, f64)): DifferentNaive);
```

This crate works around this by using a private named tuple instead of the primitive tuple, so that it is guaranteed that downstream crates will not test types that contain this named tuple.

# Known problems / quirks

1. Using both [`SameTypeAs`] and [`NotSameTypeAs`] to implement two impls for the same type will give: `error[E0119]: conflicting implementations`, probably due to the current limitations of Rust(?).

2. References to the same type, but with different lifetimes, are treated the same. This could be thought of as a "feature" if you squint hard enough.

3. For type equality in serious projects, you should probably try some [other crates](crates.io/search?q=type%20equal) by people who probably know better type theory and rust's type system.

# Unstable features

```rust
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(extended_key_value_attributes)]
```
