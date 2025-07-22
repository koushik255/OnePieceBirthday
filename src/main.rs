use chrono::Local;
use std::fs;


fn main() {
    println!("Hello, world!");
    let date = today_date();
    println!("{}",date);

    let fin = read_file();
     println!("{}",fin);
}


fn today_date() -> String{
    let today = Local::now().format("%m-%d").to_string();
    today.to_string()
     
}

fn read_file() -> String{
    let date = today_date();
    let contents = fs::read_to_string("birth.txt")
        .expect("failed to find ");
    
    let matched: String = contents
        .lines()
        .filter(|line| line.contains(&date))
        .collect();

    matched

}
