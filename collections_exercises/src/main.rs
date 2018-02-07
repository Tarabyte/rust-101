fn main() {
    test_avg();

    test_mode();

    test_median();
}

fn test_median() {
    let v = vec![1, 2, 3, 4, 5, 3, 3];

    println!(
        "Median of {:?} is {:?}",
        v,
        median(&v)
    );

    assert_eq!(median(&v), Some(3));

    let v: Vec<i32> = Vec::new();

    assert_eq!(median(&v), None);

    println!(
        "Median of {:?} is {:?}",
        v,
        median(&v)
    )
}

fn median(v: &Vec<i32>) -> Option<i32> {
    match v.len() {
        0 => None,
        len => {
            let mut sorted_copy = v.clone();
            sorted_copy.sort();

            let value = if len%2 == 0 {
                (sorted_copy[len/2] + sorted_copy[len/2])/2
            } else {
                sorted_copy[len/2]
            };

            Some(value)
        }
    }
}

fn test_mode() {
    let v = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];

    println!(
        "Mode of {:?} is {:?}",
        v,
        mode(&v)
    );

    assert_eq!(mode(&v), Some(3));

    let v: Vec<i32> = Vec::new();

    println!(
        "Mode of {:?} is {:?}",
        v,
        mode(&v)
    );
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    if v.is_empty() {
        return None
    }

    let mut counts: HashMap<i32, i32> = HashMap::new();

    for item in v {
        let count = counts.entry(*item)
            .or_insert(0);

        *count += 1;
    }


    let mut res_value: i32 = 0;
    let mut res_count: i32 = 0;

    for (value, count) in &counts {
        if *count > res_count {
            res_count = *count;
            res_value = *value;
        }
    }

    return Some(res_value);
}

fn test_avg() {
    let v = vec![1, 2, 3, 4];


    println!(
        "Average of {:?} is {}",
        v,
        avg(&v)
    );

    let v: Vec<i32> = Vec::new();

    println!(
        "Average of {:?} is {}",
        v,
        avg(&v)
    )
}

fn avg(v: &Vec<i32>) -> f32 {
    let mut sum = 0;
    let mut len = 0;

    for item in v {
        sum += *item;
        len += 1;
    }

    if len > 0 {
        sum as f32 / len as f32
    } else {
        0.0
    }
}
