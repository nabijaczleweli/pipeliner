use super::*;

#[test]
fn dumb_test() {
    let exec = Executor::new(|x: i32| x + 1);
    
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 42];
    let results = exec.work(input.into_iter()); // lazily collected w/ buffer.
    
    // Collect back into a vec: 
    let mut results: Vec<_> = results.collect();
    results.sort();
    for result in &results {
        println!("Got result: {}", result);
    }
    
    assert!(results[0] == 2);
    assert!(results[results.len()-1] == 43);
    
}

#[test]
#[should_panic(expected="Worker thread panicked with message: [I don't like the number 14]")]
fn test_panic() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 42];
    let exec = Executor::new(|x: i32| {
        if x == 14 {
            panic!("I don't like the number {}", x);
        }
        return x * 2;
    });

    let results = exec.work(input.into_iter());
    for result in results {
        println!("Got result: {}", result);
    }
    
}