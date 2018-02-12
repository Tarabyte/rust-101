#[derive(PartialEq, Debug)]
struct Shoe<'a> {
    size: u32,
    style: &'a str,
}

impl<'a> Shoe<'a> {
    fn new(size: u32, style: &str) -> Shoe {
        Shoe {
            size,
            style,
        }
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    to: u32,
    value: u32,
}

impl Counter {
    fn new(to: u32) -> Counter {
        Counter {
            to,
            value: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.value < self.to {
            let value = self.value;
            self.value += 1;

            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total = v1_iter.sum::<u32>();

        assert_eq!(total, 6);
    }

    #[test]
    fn collect_to_vec() {
        let v1 = vec![1, 2, 3];

        let v2 = v1.iter()
            .map(|x| x + 1)
            .collect::<Vec<_>>();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filter_shoes_by_size() {
        let sizes = vec![10, 13, 10];
        let styles = vec![
            "sneakers",
            "sandal",
            "boot"
        ];

        let shoes = sizes.iter()
            .zip(styles.iter())
            .map(|(size, style)| Shoe::new(*size, style))
            .collect::<Vec<_>>();

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe::new(10, "sneakers"),
                Shoe::new(10, "boot")
            ]
        );
    }

    #[test]
    fn counter_iterator() {
        let counter = Counter::new(5);

        assert_eq!(
            counter.collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new(6).zip(Counter::new(7).skip(1))
                                    .map(|(a, b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
        assert_eq!(
            1*2*0 + 2*3 + 3*4 + 4*5*0 + 5*6,
            sum
        );
    }
}
