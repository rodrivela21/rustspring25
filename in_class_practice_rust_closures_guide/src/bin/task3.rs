// Using map and collect
fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Using a for loop
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    // Using map and collect: multiply each number by 2
    let doubled = process_vector_with_map(numbers.clone(), |x| x * 2);

    // Using map and collect: if number > 2, replace with 0, else keep number
    let replaced = process_vector_with_map(numbers.clone(), |x| if x > 2 { 0 } else { x });

    println!("Doubled (map): {:?}", doubled);
    println!("Replaced (map): {:?}", replaced);

    // Using a for loop: multiply each number by 2
    let doubled_loop = process_vector_with_for_loop(numbers.clone(), |x| x * 2);

    // Using a for loop: if number > 2, replace with 0, else keep number
    let replaced_loop = process_vector_with_for_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled (for loop): {:?}", doubled_loop);
    println!("Replaced (for loop): {:?}", replaced_loop);
}