fn main() {

}

fn get_primitive_iterable_example() {
    let vector_example: Vec<&str> = vec!["nexus", "bagels", "is", "gay"];
    println!("{:?}", vector_example);

    let array_example: [&str; 4] = ["nexus", "bagels", "is", "gay"];
    println!("{:?}", array_example);

    let tuple_example: (&str, &str, &str, &str) = ("nexus", "bagels", "is", "gay");
    println!("{:?}", tuple_example);
}