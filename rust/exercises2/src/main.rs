
use std::collections::HashMap;

fn main() {

    let mut arr = vec![1, 2, 2, 2, 3];

    println!(" mean of arr is {}", mean(&arr));
    println!(" median of arr is {}", median(&mut arr));
    println!(" mode of arr is {}", mode(&mut arr));

    println!("{:?}", two_sum(vec![2,7,11,15], 9));

    book_reviews();
}

fn book_reviews() {
    let mut book_reviews = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book".to_string(),
    );

    let to_find = ["Pride and Prejudice", "Adventures of Huckleberry Finn"];
    book_review_print(&to_find, book_reviews);
}

fn book_review_print(to_find: &[&str], book_reviews: HashMap<String, String>) {
    for &book in to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }
}

fn mean(nums: &[i32]) -> f32 {
    nums.iter().sum::<i32>() as f32 / nums.len() as f32
}

fn median(nums: &mut [i32]) -> i32 {
    // sort, then find mid point using length
    nums.sort();
    let mid = nums.len() / 2;
    nums[mid]
}

fn mode(nums: &[i32]) -> i32 {
    let mut occurences = HashMap::new();

    for &value in nums {
        *occurences.entry(value).or_insert(0) += 1;
    }

    occurences
        .into_iter()
        .max_by_key(|&(_, c)| c)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
    let mut map = HashMap::new();

    for (index, element) in nums.iter().enumerate() {
        map.insert(*element, index as i32);
    }

    for (index, element) in nums.iter().enumerate() {
        let expected_no = target - element;

        if let Some(&i) = map.get(&expected_no) {
            if i != index as i32 {
                return vec![index as i32, i];
            }
        }
    }

    return vec![];
}
