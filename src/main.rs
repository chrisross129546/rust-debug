

fn main() {
    // get_memory_efficient_string();
    // vector__()
    // compare();
    // get_continued_loop();
    xd();
}

fn get_primitive_iterable_example() {
    let vector_example: Vec<&str> = vec!["nexus", "bagels", "is", "gay"];
    println!("{:?} {}", vector_example, vector_example.capacity());

    let array_example: [&str; 4] = ["nexus", "bagels", "is", "gay"];
    println!("{:?}", array_example);

    let tuple_example: (&str, &str, &str, &str) = ("nexus", "bagels", "is", "gay");
    println!("{:?}", tuple_example);
}
fn get_vector_collection(range: [i8; 2]) {
    if range[0] > range[1] { return; }

    let vector: Vec<i8> = (range[0]..=range[1]).collect();
    let vector_reference_memory: &[i8] = &vector;
    println!("{:?} {:?}", vector, vector_reference_memory);
}
fn get_memory_efficient_string() {
    let inefficient_string: String = String::from("i like javascript");

    let mut semi_efficient_string: String = String::with_capacity(25);
    semi_efficient_string.push_str("i like javascript");

    let efficient_string: &str = "hello world";
    println!("{} {} {} {}", inefficient_string, semi_efficient_string, efficient_string, efficient_string.len());
}
fn array__() -> [i32; 3] {
    // all the types have to be the same
    [27, 36, 42]
}
fn tuple__() -> (i32, char, u32) {
    // collection with different values, however they have to all be predetermined and this is probably growable
    (27, 'a', 3283)
}
fn vector__() {
    let mut vec: Vec<i32> = vec![23, 14, 6];
    // infers 3 bytes of memory for the vector
    println!("{}", vec.capacity());
    // should result in 3
    vec.push(2);
    // should result in the capacity being twice as much as it was before
    // every time you push something into a vector it doubles the capacity in memory from the previous size
    println!("{}", vec.capacity())
}
fn compare() -> bool {
    let a: &str = "Hello world";
    let c: &str = "Hello world";
    return a == c;
}
fn get_continued_loop() {
    let mut count: u8 = 0;
    'label: loop {
        if (count == 255) {
            break 'label;
        }

        count += 1;
    }

    println!("{}", count);
}

fn arrow_function() {
    let x = |arr: [u8; 3]| -> [u8; 3] {
        return arr.map(|number: u8| number + 4);
    };

    print!("{:?}", x([1, 2, 3]));
}