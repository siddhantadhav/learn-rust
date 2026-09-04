fn change_ownership(s: String) -> usize {
    let s = "hello_changes";
    s.len()
}

fn change_ownership_ref(s: &String) -> usize {
    let s = "hello_changes"; // This line creates a new string slice, but does not change the ownership of the original string. 
                             // The original string is still owned by the caller and the scope of new s is limited to this function.
    s.len()
}

// This function will not compile because it tries to mutate an immutable reference.
// fn update_string(s: &String) {
//     s.push_str(", world");
// }

fn update_string_mut(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    // let s1 = String::from("hello");
    // let result = change_ownership(s1);
    // println!("The length of the string is: {}", result);
    // println!("The original string is: {}", s1); // This line will cause a compile-time error because s1 has been moved to the function.

    // let result_ref = change_ownership_ref(&s1);
    // println!("The length of the string is: {}", result_ref);
    // println!("The original string is: {}", s1); // This line will work because s1 was passed by reference.

    let mut s2 = String::from("hello");
    // update_string(&s2);
    // println!("The updated string is: {}", s2); // This line will cause a compile-time error because s2 is borrowed as immutable in the function, but we are trying to mutate it.
    let s3 = &mut s2; // will work because s2 is mutable and we are passing a mutable reference to the function.
    let s4 = &mut s2; // will not work because s2 is already borrowed as mutable in the previous line, and we cannot have multiple mutable references to the same variable at the same time.
    println!("The updated string is: {}", s3);
    // println!("The updated string is: {}", s4);
    // s4.push_str(", world"); // will not work because s4 is already borrowed as mutable in the previous line, and we cannot have multiple mutable references to the same variable at the same time.
    update_string_mut(&mut s2);
    println!("The updated string is: {}", s2);
}
