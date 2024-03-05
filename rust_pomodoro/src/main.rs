use chrono::Local;
use std::thread;

fn main() {
    start_pomodoro(1, 1, 4);
}


fn start_timer (minutes: i64, seconds: i64){
    let mut time_now = Local::now();
    let time_end = time_now + chrono::Duration::minutes(minutes) + chrono::Duration::seconds(seconds);

    println!("Time now: {}, Finish time: {}", time_now.format("%H:%M.%S"), time_end.format("%H:%M.%S"));

    let mut timer_diff = time_end - time_now;

    while timer_diff.num_seconds() > 0 {
        time_now = Local::now();
        timer_diff = time_end - time_now;

        println!("Time remaining: {}:{:02}", timer_diff.num_minutes(), timer_diff.num_seconds() % 60);
        thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("Times up!");
}

fn start_pomodoro (work_len: i64, break_len: i64, cycles: i32){
    for i in 0..cycles {
        println!("Start working for {} minutes.", work_len);
        start_timer(work_len, 0);
        println!("{} minute break! Stand up, move around.", break_len);
        start_timer(break_len, 0);
    }
    println!("Pomodoro finished");
}