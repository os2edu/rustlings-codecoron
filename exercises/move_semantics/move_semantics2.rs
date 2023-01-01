// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)
// copy from https://github.com/aesteve/rustlings/blob/4ac2e7f5ea545ba7a0ca9fc7b80aa0d035194a5c/exercises/move_semantics/move_semantics2.rs

fn main() {
    // solution 1: Just clone vec0 before calling fill
    /*
    let vec0 = Vec::new();
    let vec_cloned = vec0.clone();
    let mut vec1 = fill_vec(vec_cloned);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
     */

    // Solution2: Change fill so that it borrows its argument instead of taking ownership of it
    // --> fill_vec_borrow
    /*
    let vec0 = Vec::new();
    let mut vec1 = fill_vec_borrow(&vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
     */

    // Solution3: mutably borrow vec in fill_vec
    let mut vec0 = Vec::new();

    fill_vec_mutably_borrow(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec0.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}


fn fill_vec_borrow(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}


fn fill_vec_mutably_borrow(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}