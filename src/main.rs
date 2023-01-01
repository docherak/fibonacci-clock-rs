extern crate chrono;

use chrono::{Local, DateTime, Timelike};
use std::collections::HashMap;

// Struct for assigning squares
struct Fibonacci {
    five: bool,
    three: bool,
    two: bool,
    one_a: bool,
    one_b: bool,
}

impl Fibonacci {
    fn default() -> Fibonacci {
        Fibonacci {
            five: false,
            three: false,
            two: false,
            one_a: false,
            one_b: false,
        }
    }

    fn change_value(&mut self, cond: i32) {
        match cond {
            5 => self.five = true,
            3 => self.three = true,
            2 => self.two = true,
            1 => self.one_a = true,
            _ => println!("Mismatch!"),
        }
    }

    fn distribute_values(&mut self, value: i32, is_hour: bool) {
        let mut tmp: i32 = if !is_hour { 
            value / 5
        } else {
            value
        };
        let fib_seq: [i32; 5] = [5, 3, 2, 1, 1];
        while tmp != 0 {
            for v in fib_seq.iter() {
                if tmp / v >= 1 {
                    if self.one_a == true && *v == 1 {
                        self.one_b = true;
                    }
                    self.change_value(*v);
                    tmp -= v;
                }
            }
        }
    }
}

// Struct for colorizing + color enum
#[derive(Debug)]
enum Color {
    Red,    // hours
    Green,  // minutes
    Blue,   // hours+mins   
    White,  // none
}

struct Colorized {
    five: Color,
    three: Color,
    two: Color,
    one_a: Color,
    one_b: Color,
}

impl Colorized  {
    fn default() -> Colorized {
        Colorized {
            five: Color::White,
            three: Color::White,
            two: Color::White,
            one_a: Color::White,
            one_b: Color::White,
        }
    }

    fn matcher(&mut self, num_name: &String) -> i32 {
        match num_name.as_str() {
            "Five" => 5,
            "Three" => 3,
            "Two" => 2,
            "One_a" => 1,
            "One_b" => 0,
            _ => -1,
        }
    }

    fn colorize(&mut self, f_h: &mut Fibonacci, f_m: &mut Fibonacci) {

        let mut hash_h: HashMap<String, bool> = HashMap::new();
        hash_h.insert(String::from("Five"), f_h.five);
        hash_h.insert(String::from("Three"), f_h.three);
        hash_h.insert(String::from("Two"), f_h.two);
        hash_h.insert(String::from("One_a"), f_h.one_a);
        hash_h.insert(String::from("One_b"), f_h.one_b);
        
        let mut hash_m: HashMap<String, bool> = HashMap::new();
        hash_m.insert(String::from("Five"), f_m.five);
        hash_m.insert(String::from("Three"), f_m.three);
        hash_m.insert(String::from("Two"), f_m.two);
        hash_m.insert(String::from("One_a"), f_m.one_a);
        hash_m.insert(String::from("One_b"), f_m.one_b);

        for (key_h, value_h) in &hash_h {
            for (key_m, value_m) in &hash_m { 
                if (key_h.eq(key_m)) && (*value_h && *value_m) {
                    let num = self.matcher(key_h);
                    match num {
                        5 => self.five = Color::Blue,
                        3 => self.three = Color::Blue,
                        2 => self.two = Color::Blue,
                        1 => self.one_a = Color::Blue,
                        0 => self.one_b = Color::Blue,
                        _ => println!("Color not assigned!"),
                    }
                } else if (key_h.eq(key_m)) && (*value_h) {
                    let num = self.matcher(key_h);
                    match num {
                        5 => self.five = Color::Red,
                        3 => self.three = Color::Red,
                        2 => self.two = Color::Red,
                        1 => self.one_a = Color::Red,
                        0 => self.one_b = Color::Red,
                        _ => println!("Color not assigned!"),
                    }
                } else if (key_h.eq(key_m)) && (*value_m) {
                    let num = self.matcher(key_m);
                    match num {
                        5 => self.five = Color::Green,
                        3 => self.three = Color::Green,
                        2 => self.two = Color::Green,
                        1 => self.one_a = Color::Green,
                        0 => self.one_b = Color::Green,
                        _ => println!("Color not assigned!"),
                    }
                }
            }
        }
    }
}


fn main() {

    let mut fib_hours = Fibonacci::default();
    let mut fib_minutes = Fibonacci::default();
    let mut colour = Colorized::default();
    let now: DateTime<Local> = Local::now();

    // Hours in 0-12 format    
    //let mut hour = 10;
    let mut hour = now.hour12().1 as i32;
    //let mut minute = 55;
    let mut minute = now.minute() as i32;
    
    // Calculate time for 5min intervals
    while minute % 5 != 0 {
        minute += 1;
    }

    // Border values
    if minute == 60 {
        if hour + 1 == 13 {
            hour = 1;
        } else {
            hour += 1;
        }
        minute = 0;
    } else if minute > 60 {
        if hour + 1 == 13 {
            hour = 1;
        } else {
            hour += 1;
        }
        minute = 5;
    }

    println!("Current time: {}:{}", hour, minute);
    
    // Distribute values for hours and minutes
    fib_hours.distribute_values(hour, true);
    fib_minutes.distribute_values(minute, false);
    
    colour.colorize(&mut fib_hours, &mut fib_minutes);
    println!("5:\t{:?}", colour.five);
    println!("3:\t{:?}", colour.three);
    println!("2:\t{:?}", colour.two);
    println!("1:\t{:?}", colour.one_a);
    println!("1:\t{:?}", colour.one_b);
}
