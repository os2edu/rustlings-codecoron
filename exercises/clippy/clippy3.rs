// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

// //

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //because of this check,my_option.unwrap() will panic
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    //Checks for binding a unit value.
    //A unit value cannot usefully be used anywhere. So binding one is kind of pointless.
    //TODO reset(0,5) return what?
    // return ()
    let mut my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
