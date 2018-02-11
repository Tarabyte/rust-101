use std::collections::HashMap;
use std::hash::Hash;
use std::marker::Copy;


pub struct Cacher<T, U, V>
    where   T: Fn(U) -> V
{
    computation: T,
    values: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
    where   T: Fn(U) -> V,
            U: Eq + Hash + Copy,
            V: Copy
{
    pub fn new(computation: T) -> Self {
        let values = HashMap::new();

        Cacher {
            computation,
            values,
        }
    }

    pub fn value(&mut self, arg: U) -> V {
        let computation = &self.computation;

        *self.values.entry(arg).or_insert_with(move || {
            println!("Performing actuall computation");
            computation(arg)
        })

    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_argument_cacher() {
        let mut cached = Cacher::new(|x| x);

        assert_eq!(cached.value(1), 1);
    }

    #[test]
    fn different_arguments() {
        let mut cached = Cacher::new(|x| x);

        let value1 = cached.value(1);
        assert_eq!(value1, 1);
        let value2 = cached.value(2);

        assert_eq!(value2, 2);
    }

    #[test]
    fn caching_strings() {
        let mut upper_case = Cacher::new(|x: &str| x.len());

        let value1 = upper_case.value("afkd");
        assert_eq!(value1, 4);

    }

    #[test]
    fn should_call_once() {
        let mut cached = Cacher::new(|x| {
            println!("Should only be called once");

            x * x
        });

        assert_eq!(cached.value(3), 9);
        assert_eq!(cached.value(3), 9);
    }
}