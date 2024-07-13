use std::io;
use rand::Rng;
use std::{thread, time::Duration};
use progressed::{ProgressBar, ProgressBarStyle};
use colored::Colorize;

fn main() {
    println!(
        "{}\n",
        "OSINT INF".yellow()
    );
    println!(
        "{} {}      {} {}\n{} {}        {} {}\n",
        ".1".yellow(),
        "Number".blue(),
        ".2".yellow(),
        "Meli".blue(),
        ".3".yellow(),
        "Bank".blue(),
        ".4".yellow(),
        "City".blue()
    );
    println!(
        "{} {} {}",
        "(".blue(),
        "Tool".green(),
        ")>".blue()
    );

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "number" || input.trim() == "1" {
        println!(
            "{}{}{}{}",
            "number".green(),
            "/".yellow(),
            "(Name)".blue(),
            ">".yellow()
        );

        let mut name = String::new();
        io::stdin().read_line(&mut name)
            .expect("Failed to read line");

        let choices = vec!["+989156564543", "+989154343512", "+989335677865", "+989023039454", "+989378987878"];
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..choices.len());
        let random_choice = choices[random_index];
        for _ in ProgressBar::new(0..100)
            .set_style(ProgressBarStyle::smooth())
            .set_title("receiving: ")
            {
                   thread::sleep(Duration::from_millis(100));
            }
        println!(
            "Data : [name > {}, number > {}]",
            name.trim(),
            random_choice
        );
    } else if input.trim() == "meli" || input.trim() == "2" {
        println!(
            "{}{}{}{}",
            "meli".green(),
            "/".yellow(),
            "(Name)".blue(),
            ">".yellow()
        );
        let mut name1 = String::new();
        io::stdin().read_line(&mut name1)
            .expect("Failed to read line");

        let choices1 = vec!["0929876541", "7038473727", "0929876543", "0726521987", "0928765431"];
        let mut rng1 = rand::thread_rng();
        let random_index1 = rng1.gen_range(0..choices1.len());
        let random_choice1 = choices1[random_index1];
        for _ in ProgressBar::new(0..100)
            .set_style(ProgressBarStyle::smooth())
            .set_title("receiving: ")
            {
                   thread::sleep(Duration::from_millis(100));
            }
        println!(
            "Data : [name > {}, mali > {}]",
            name1.trim(),
            random_choice1
        );
    } else if input.trim() == "bank" || input.trim() == "3" {
        println!(
            "{}{}{}{}",
            "bank".green(),
            "/".yellow(),
            "(Name)".blue(),
            ">".yellow()
        );
        let mut bank = String::new();
        io::stdin().read_line(&mut bank)
            .expect("Failed to read line");

        let choices2 = vec!["1780-1987-1987-1212", "9208-2635-3676-3218", "9090-9876-2309-3654", "3232-2329-2878-2645", "9987-6514-0870-7935"];
        let mut rng2 = rand::thread_rng();
        let random_index2 = rng2.gen_range(0..choices2.len());
        let random_choice2 = choices2[random_index2];
        for _ in ProgressBar::new(0..100)
            .set_style(ProgressBarStyle::smooth())
            .set_title("receiving: ")
            {
                   thread::sleep(Duration::from_millis(100));
            }
        println!(
            "Data : [name > {}, bank > {}]",
            bank.trim(),
            random_choice2
        );
    } else if input.trim() == "city" || input.trim() == "4" {
        println!(
            "{}{}{}{}",
            "city".green(),
            "/".yellow(),
            "(Name)".blue(),
            ">".yellow()
        );
        let mut city = String::new();
        io::stdin().read_line(&mut city)
            .expect("Failed to read line");

        let choices3 = vec!["Mashhad", "Ghazvin", "Tehran", "Boshehr", "Tabriz"];
        let mut rng3 = rand::thread_rng();
        let random_index3 = rng3.gen_range(0..choices3.len());
        let random_choice3 = choices3[random_index3];
        for _ in ProgressBar::new(0..100)
            .set_style(ProgressBarStyle::smooth())
            .set_title("receiving: ")
            {
                   thread::sleep(Duration::from_millis(100));
            }
        println!(
            "Data : [name > {}, city > {}]",
            city.trim(),
            random_choice3
        )
    } else {
        println!("! command {}",input);
    }
}
