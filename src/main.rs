use std::io;
use rand::Rng;
use std::time::{Duration, Instant};

fn initial_log() {
    println!("\nPlease set a level:");
    println!("1 - Easy");
    println!("2 - Medium");
    println!("3 - Hard");
    println!("exit - to exit from the test");
    println!("help - to review input options\n")
}

fn warning_log() {
    println!("Please input a correct number (1, 2 or 3), exit or help\n");
}

fn warning_answer_log() {
    println!("Please input a number or exit\n");
}

fn score_to_mark(score: u32) -> String {
    if score == 0 {
        String::from("You are safe from Math")
    } else if score == 1 {
        String::from("Beginner")
    } else if score == 2 {
        String::from("Pre-PhD")
    } else if score == 3 {
        String::from("Expert")
    } else if score == 4 {
        String::from("Einstein's teacher")
    } else if score == 5 {
        String::from("You are the Math")
    } else {
        String::from("Unknown score")
    }
}

fn print_line(label: String, length: usize, elems: [String; 5], size: [usize; 5]) {
    let mut output_line: String = String::from("|");
    output_line += &(label + &"|");
    for index in 0..length {
        output_line += &(adapt(&elems[index].to_string(), size[index]) + &"|");
    }
    println!("{output_line}");
}

fn print_and_return_results(correct: [bool; 5], times: [f32; 5], step: usize, level: u32) -> (u32, f32) {
    let mut max_length_cell: [usize; 5] = [3; 5];
    let mut correct_string: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let times_values: [f32; 5] = times;
    let mut times_string: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let mut score = 0;
    let mut total_time: f32 = 0.0;
    let num_array: [String; 5] = [String::from("1"), String::from("2"), String::from("3"), String::from("4"), String::from("5")];
    let mut whole_amount_of_cells: usize = 9 + 7;
    for index in 0..step {
        total_time += times[index];
        times_string[index] = String::from(times_values[index].to_string());
        max_length_cell[index] = times[index].to_string().len() + 4;
        whole_amount_of_cells += max_length_cell[index];
        if correct[index] {
            score += 1;
            correct_string[index] = String::from("+");
        } else {
            correct_string[index] = String::from("-");
        }
    }
    whole_amount_of_cells += (5 - step) * 3;
    println!("Results on level {}:", level);
    println!("{}", String::from("-").repeat(whole_amount_of_cells));
    print_line(String::from("  Task   "), 5, num_array, max_length_cell);
    println!("{}", String::from("-").repeat(whole_amount_of_cells));
    print_line(String::from(" Correct "), 5, correct_string, max_length_cell);
    println!("{}", String::from("-").repeat(whole_amount_of_cells));
    print_line(String::from("  Time   "), 5, times_string, max_length_cell);
    println!("{}", String::from("-").repeat(whole_amount_of_cells));
    let mut average_time: f32 = 0.0;
    if step > 0 {
        average_time = ((total_time / step as f32) * 100.0).round() / 100.0;
    }
    println!("\nTotal score: {score}");
    println!("Average time: {}", average_time );
    println!("Mark: {}\n", score_to_mark(score));
    (score, average_time)
}

fn update_game_highscore(max_score: &mut [u32; 3], min_time: &mut [f32; 3], level: usize, score: u32, time: f32) -> bool {
    if max_score[level - 1] < score {
        max_score[level - 1] = score;
        min_time[level - 1] = time;
        return true;
    } else if max_score[level - 1] == score && min_time[level - 1] > time {
        min_time[level - 1] = time;
        return true;
    }
    return false;
}

fn adapt(elem: &str, max_length: usize) -> String {
    let mut output_string: String = String::new();
    let first_part: usize = (max_length - elem.len()) / 2 + (max_length - elem.len()) % 2;
    let second_part: usize = (max_length - elem.len()) / 2;
    output_string += &" ".repeat(first_part);
    output_string += elem;
    output_string += &" ".repeat(second_part);
    output_string
}

fn print_highscore(max_score: [u32; 3], min_time: [f32; 3]) {
    let mut print: bool = false;
    let mut max_length_cell: [usize; 5] = [0; 5];
    let mut scores_string: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let mut times_string: [String; 5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];
    let num_array: [String; 5] = [String::from("1"), String::from("2"), String::from("3"), String::from(""), String::from("")];
    for index in 0..3 {
        if min_time[index] > 0.0 {
            print = true;
        }
        scores_string[index] = max_score[index].to_string();
        times_string[index] = min_time[index].to_string();
        max_length_cell[index] = min_time[index].to_string().len() + 4;
    }

    if print {
        println!("\nHigh score on each level during this game:");
        let whole_amount_of_cells: usize = 7 + 5 + max_length_cell[0] + max_length_cell[1] + max_length_cell[2];
        println!("{}", String::from("-").repeat(whole_amount_of_cells));
        print_line(String::from(" Level "), 3, num_array, max_length_cell);
        println!("{}", String::from("-").repeat(whole_amount_of_cells));
        print_line(String::from(" Score "), 3, scores_string, max_length_cell);
        println!("{}", String::from("-").repeat(whole_amount_of_cells));
        print_line(String::from(" Time  "), 3, times_string, max_length_cell);
        println!("{}", String::from("-").repeat(whole_amount_of_cells));
    }
}

fn main() {
    println!("Welcome to the Mind calculating test");

    let mut max_score: [u32; 3] = [0; 3];
    let mut min_time: [f32; 3] = [0.0; 3];

    loop {
        initial_log();
    
        let mut level: u32;
        
        loop {
            let mut level_input = String::new();

            io::stdin()
                .read_line(&mut level_input)
                .expect("Internal reading error");

            level = match level_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if level_input.trim() == "exit" {
                        print_highscore(max_score, min_time);
                        println!("\nSuccessfully exited from the test!");
                        return;
                    } else if level_input.trim() == "help" {
                        initial_log();
                        continue;
                    } else if level_input.trim() == "" {
                        continue;
                    } else {
                        warning_log();
                        continue;
                    }
                },
            };
            if level >= 1 && level <= 3 {
                break;
            } else {
                warning_log();
            }
        }

        println!("\nYou put a level {level}");
        if level == 1 {
            println!("It is an Easy level");
        } else if level == 2 {
            println!("It is a Medium level");
        } else {
            println!("It is a Hard level");
        }
        println!();

        println!("Get ready to the Mind calculating test on level {level}");
        println!("You need to solve 5 math equations");
        println!();
        println!("When you would ready press Enter or exit");

        let mut ready_to_test:String = String::new();
        io::stdin()
            .read_line(&mut ready_to_test)
            .expect("Internal reading error");

        if ready_to_test.trim() == "exit" {
            print_highscore(max_score, min_time);
            println!("\nSuccessfully exited from the test!");
            return;
        }

        println!("Let's start\n");
        let mut list_a: [u32; 5] = [0; 5];
        let mut list_b: [u32; 5] = [0; 5];
        let mut correct: [bool; 5] = [false; 5];
        let mut times: [f32; 5] = [0.0; 5];
        let base: u32 = 10;
        let mut step: usize = 0;
        let mut exited: bool = false;
        while step < 5 && !exited {
            let a = rand::thread_rng().gen_range(base.pow(level - 1)..=base.pow(level));
            let b = rand::thread_rng().gen_range(base.pow(level - 1)..=base.pow(level));
            list_a[step] = a;
            list_b[step] = b;
            let answer: u32;
            let time_spent: Duration;
            loop {
                let time_calculation = Instant::now();
                println!("{a} + {b} = ?\n");
                let mut answer_str:String = String::new();
                io::stdin()
                    .read_line(&mut answer_str)
                    .expect("Internal reading error");

                answer = match answer_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        if answer_str.trim() == "exit" {
                            println!("\nSuccessfully exited from the round!");
                            exited = true;
                            break;
                        } else if answer_str.trim() == "" {
                            continue;
                        } else {
                            warning_answer_log();
                            continue;
                        }
                    },
                };
                time_spent = time_calculation.elapsed();
                println!("Time spent: {:.2}", time_spent.as_secs_f32());
                times[step] = (time_spent.as_secs_f32() * 100.0).round() / 100.0;
                correct[step] = a + b == answer;
        
                if correct[step] {
                    println!("\nCorrect answer!\n");
                } else {
                    println!("\nIncorrect answer!");
                    println!("Correct answer is {}\n", a + b);
                }
                step += 1;
                break;
            }
        }

        let score: u32;
        let time: f32;
        (score, time) = print_and_return_results(correct, times, step, level);

        let is_updated: bool = update_game_highscore(&mut max_score, &mut min_time, level as usize, score, time);

        if is_updated {
            println!("You beat a new record during this game:");
            println!("Level: {level}");
            println!("Score: {score}");
            println!("Average time: {time}");
        }

        println!("\nDo you want to repeat a test?");

        loop {
            println!("Please input repeat or exit\n");

            let mut repeat:String = String::new();
            io::stdin()
                .read_line(&mut repeat)
                .expect("Internal reading error");

            if repeat.trim() == "repeat" {
                break;
            } else if repeat.trim() == "exit" {
                print_highscore(max_score, min_time);
                println!("\nSuccessfully exited from the test!");
                return;
            } else {
                continue;
            }
        }
    }
}