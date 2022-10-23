use rand::{thread_rng, Rng};

fn main() {
    // Calculating fibonacci
    let mut memory: [u64; 100] = [ 0; 100];
    let n: u64 = 80;
    println!("Fibonacci of {n} is {}", fib(n, &mut memory));

    // Sorting vector
    let mut numbers: Vec<u8> = gen_numbers(10);
    println!("Unsorted: {:?}", numbers);
    let sorted = bubble_sort(&mut numbers);
    println!("Sorted: {:?}", sorted);

    // Closures
    let incr = |i: i32| -> i32 { i + 1 };
    println!("{}, {}", incr(3), incr(-40));

    let do_things = || -> () {
        // Do a lot of things
        let mut tasks: Vec<String> = Vec::new();
        tasks.push("wash dishes".to_string());
        tasks.push("fly to the moon".to_string());

        for task in tasks.iter_mut() {
            task.make_ascii_uppercase();
        }
        println!("Tasks: {:?}", tasks);
    };
    do_things();

    // HOF (High Order Functions)
    let sum_of_squared_odd_numbers: u32 = (0..100)
        .map(|x| x*x)
        .take_while(|&squared| squared < 26)
        .filter(|&squared| squared % 2 == 1)
        .sum();
    // 1² + 3² + 5² == 1 + 9 + 25 == 35
    assert_eq!(sum_of_squared_odd_numbers, 35)

}

fn fib(n: u64, memory: &mut [u64]) -> u64{
    // Calculates fibonacci
    if n < 2 {
        return 1
    }
    let n_usize: usize = n as usize;
    if memory[n_usize] == 0 {
        memory[n_usize] = fib(n-2, memory) + fib(n-1, memory);
    }
    memory[n_usize]
}

fn gen_numbers(n: u8) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let mut rng = thread_rng();
    let mut random_nb: u8;
    for _ in 1..=n{
        random_nb = rng.gen_range(0..100);
        vec.push(random_nb);
    }
    vec
}

fn bubble_sort(numbers: &mut Vec<u8>) -> &mut Vec<u8> {
    for _ in 1..numbers.len() {
        for i in 0..numbers.len()-1 {
            if numbers[i] > numbers[i+1] {
                let temp = numbers[i];
                numbers[i] = numbers[i+1];
                numbers[i+1] = temp;
            }
        }
    }
    numbers
}