use std::mem::size_of;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    child: Option<Rc<Node>>,
}

fn greater_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn greater_string(a: &str, b: &str) -> String {
    if a.len() >= b.len() {
        a.to_string()
    } else {
        b.to_string()
    }
}

fn moved_value_example() {
    let a = "Hello, world!".to_string();
    let b = a.clone();
    let c = a;

    println!("1. clone before move => b = {b}, c = {c}");
}

fn move_after_borrows_end_example() {
    let a = "Hello, world!".to_string();

    {
        let b = &a;
        let c = &a;
        println!("2. borrowed first => b = {b}, c = {c}");
    }

    let d = a;
    println!("2. moved after borrows ended => d = {d}");
}

fn mutable_vs_immutable_borrow_example() {
    let mut a = "Hello, world!".to_string();

    {
        let b = &a;
        let c = &a;
        println!("3. immutable borrows => b = {b}, c = {c}");
    }

    let d = &mut a;
    d.push_str("!!!");
    println!("3. mutable borrow after immutable borrows ended => d = {d}");
}

fn lifetime_example() {
    let a = "Hello, world!".to_string();
    let b = "Bye, world".to_string();

    let owned = greater_string(&a, &b);
    let borrowed = greater_str(&a, &b);

    println!("4. returned owned String => {owned}");
    println!("4. returned borrowed &str => {borrowed}");
}

fn rc_example() {
    let leaf = Rc::new(Node {
        value: 100,
        child: None,
    });

    let parent1 = Node {
        value: 1,
        child: Some(Rc::clone(&leaf)),
    };
    let parent2 = Node {
        value: 2,
        child: Some(Rc::clone(&leaf)),
    };

    println!("5. ref count after sharing = {}", Rc::strong_count(&leaf));
    println!("5. parent1 = {:?}", parent1);
    println!("5. parent2 = {:?}", parent2);
    println!("5. leaf value = {}", leaf.value);

    drop(parent1);
    println!("5. ref count after dropping parent1 = {}", Rc::strong_count(&leaf));

    drop(parent2);
    println!("5. ref count after dropping parent2 = {}", Rc::strong_count(&leaf));
}

fn as_ref_example() {
    let a = Some("a".to_string());

    if let Some(v) = a.as_ref() {
        println!("6. borrowed through as_ref() => {v}");
    }

    let b = a;
    println!("6. original Option moved later => {:?}", b);
}

fn box_example() {
    let large_buffer = Box::new([0u8; 1024 * 1024]);
    let boxed_vec = Box::new(vec![10, 20, 30, 40]);

    println!("7. boxed array length = {}", large_buffer.len());
    println!("7. size_of::<Vec<i32>>() = {}", size_of::<Vec<i32>>());
    println!("7. size_of::<Box<Vec<i32>>>() = {}", size_of::<Box<Vec<i32>>>());
    println!("7. boxed vec contents = {:?}", boxed_vec);
}

fn str_vs_string_example() {
    let line = "AAAA|BBBB|CCCC";

    let mut fields: Vec<String> = line.split('|').map(str::to_string).collect();

    for index in [0usize, 2usize] {
        let replacement = 1234.to_string();
        fields[index] = replacement;
    }

    println!("8. owned fields you can replace safely => {:?}", fields);
}

fn main() {
    moved_value_example();
    move_after_borrows_end_example();
    mutable_vs_immutable_borrow_example();
    lifetime_example();
    rc_example();
    as_ref_example();
    box_example();
    str_vs_string_example();
}