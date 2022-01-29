fn main() {
    let _a: [i32; 3] = [1,2,3];

    // vector can grow in size
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for i in &v {
        println!("{}", i);
    }

    // updating
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("After update {}", i);
    }

    // vector macro to initialize during declaration
    {
        let mut v2 = vec![1,2,3];

        // let third = v[2] is basically i32 type so this is copy
        let third = &v2[2];
        println!("The third element is, {}", third);
        v2.push(9);

        // problem here
        //println!("The third element is, {}", third);

        match v.get(2) {
            Some(third) => println!(" The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    spreadsheet_fn();
}


fn spreadsheet_fn() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("Yes this is a integer {}", i),
        _ => println!("Not a integer"),
    }
}
