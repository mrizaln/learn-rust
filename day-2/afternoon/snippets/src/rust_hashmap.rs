use std::collections::HashMap;

pub fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckelberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Miserables") {
        println!(
            "We've got {} books, but Les Miserables ain't one.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{}: {} pages.", book, count),
            None => println!("{} is not present in the map.", book),
        }
    }

    println!("{page_counts:#?}");

    // get or
    let pc1 = page_counts
        .get("Harry Potter and The Sorcerer's Stone")
        .unwrap_or(&336);
    println!("{pc1}");

    // get or insert
    let pc2 = page_counts
        .entry("The Hunger Games".to_string())
        .or_insert(374);
    println!("{pc2}");

    println!("{page_counts:#?}");

    // since rust 1.56: initialize hashmap using literal array
    let page_counts2 = HashMap::from([
        ("Harry Potter and The Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts2:#?}");
}
