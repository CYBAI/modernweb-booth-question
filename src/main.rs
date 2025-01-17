extern crate termion;

use std::io;
use std::fs::File;
use termion::{color, style};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Subject {
    question: String,
    answer: String
}

fn main() {

    let questions_file = File::open("src/question.json").expect("Failed to reading file: question.json.");
    let subjects: Vec<Subject> = serde_json::from_reader(questions_file).unwrap();

    let mut count = 0;
    let length = subjects.len();
    for subject in &subjects {
        count += 1;
        loop {
            println!("{}{}{}/{} 問題：{}{}", color::Fg(color::Cyan), style::Bold, count, length, subject.question, style::Reset);
            let mut guess = String::new();
            println!("{}請輸入答案：{}", color::Fg(color::LightBlack), style::Reset);
            io::stdin().read_line(&mut guess)
                .expect("請輸入一些文字");
            if guess.trim().to_lowercase() == subject.answer {
                println!("{}答對了，你好棒 owo{}", color::Fg(color::Yellow), style::Reset);
                break;
            } else {
                println!("{}叭叭，答錯了 OAO{}", color::Fg(color::LightRed), style::Reset);
            }
        }
    }
    println!("{}你全部都答對了耶，可以領個小禮物 OwO{}", color::Fg(color::LightYellow), style::Reset);
}
