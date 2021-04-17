//!
//! This module defines helpers to allow optional Rayon usage.
//!

use rayon::prelude::*;
use rayon_cond::CondIterator;

pub const ENV_VARIABLE: &str = "EXTRACTORS_PARALLELISM";

/// Check if the EXTRACTORS_PARALLELISM env variable has been explicitly set
pub fn is_parallelism_configured() -> bool {
    std::env::var(ENV_VARIABLE).is_ok()
}

/// Get the currently set value for `EXTRACTORS_PARALLELISM` env variable
pub fn get_parallelism() -> bool {
    match std::env::var(ENV_VARIABLE) {
        Ok(mut v) => {
            v.make_ascii_lowercase();
            match v.as_ref() {
                "" | "off" | "false" | "f" | "no" | "n" | "0" => false,
                _ => true,
            }
        }
        Err(_) => true, // If we couldn't get the variable, we use the default
    }
}

/// Set the value for `EXTRACTORS_PARALLELISM` for the current process
pub fn set_parallelism(val: bool) {
    std::env::set_var(ENV_VARIABLE, if val { "true" } else { "false" })
}

/// Allows to convert into an iterator that can be executed either parallelly or serially.
///
/// The choice is made according to the currently set `EXTRACTORS_PARALLELISM` environment variable.
/// This variable can have one of the following values
///   - False => "" (empty value), "false", "f", "off", "no", "n", "0"
///   - True => Any other value
///
pub trait MaybeParallelIterator<P, S>
where
    P: ParallelIterator,
    S: Iterator<Item = P::Item>,
{
    /// Convert ourself in a CondIterator, that will be executed either in parallel or serially,
    /// based solely on the `EXTRACTORS_PARALLELISM` environment variable
    fn into_maybe_par_iter(self) -> CondIterator<P, S>;
    /// Convert ourself in a CondIterator, that will be executed either in parallel or serially,
    /// based on both the `EXTRACTORS_PARALLELISM` environment variable and the provided bool.
    /// Both must be true to run with parallelism activated.
    fn into_maybe_par_iter_cond(self, cond: bool) -> CondIterator<P, S>;
}

impl<P, S, I> MaybeParallelIterator<P, S> for I
where
    I: IntoParallelIterator<Iter = P, Item = P::Item> + IntoIterator<IntoIter = S, Item = S::Item>,
    P: ParallelIterator,
    S: Iterator<Item = P::Item>,
{
    fn into_maybe_par_iter(self) -> CondIterator<P, S> {
        CondIterator::new(self, get_parallelism())
    }

    fn into_maybe_par_iter_cond(self, cond: bool) -> CondIterator<P, S> {
        if cond {
            self.into_maybe_par_iter()
        } else {
            CondIterator::from_serial(self)
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     #[ignore]
//     fn test_maybe_parallel_iterator() {
//         let mut v = vec![1u32, 2, 3, 4, 5, 6];
// 
//         assert_eq!(v.maybe_par_iter().sum::<u32>(), 21);
//         assert_eq!(v.maybe_par_iter_mut().map(|v| *v * 2).sum::<u32>(), 42);
//         assert_eq!(v.maybe_par_iter().sum::<u32>(), 42);
//         assert_eq!(v.into_maybe_par_iter().sum::<u32>(), 42);
//     }
// }
