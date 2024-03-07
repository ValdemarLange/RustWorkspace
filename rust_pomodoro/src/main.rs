use chrono::Local;
use std::thread;


struct PomodoroTimer {
    work_length: i64,
    break_length: i64,
    cycles: i32,
}

impl PomodoroTimer {
    fn new(work_length: i64, break_length: i64, cycles: i32) -> PomodoroTimer {
        PomodoroTimer {
            work_length,
            break_length,
            cycles,
        }
    }

    fn start_timer(&self, minutes: i64, seconds: i64) {
        let time_end = Local::now() + chrono::Duration::minutes(minutes) + chrono::Duration::seconds(seconds);

        println!("Time now: {}, Finish time: {}", Local::now().format("%H:%M.%S"), time_end.format("%H:%M.%S"));

        while Local::now() < time_end {
            let timer_diff = time_end - Local::now();
            println!("Time remaining: {}:{:02}", timer_diff.num_minutes(), timer_diff.num_seconds() % 60);
            thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Times up!");
    }

    fn start(&self){
        for _ in 0..self.cycles {
            println!("Start working for {} minutes.", self.work_length);
            self.start_timer(self.work_length, 0);
            println!("{} minute break! Stand up, move around.", self.break_length);
            self.start_timer(self.break_length, 0);
    
        }
    }

}


fn main() {
    let my_pomodoro_timer = PomodoroTimer::new(25, 5, 4);
    my_pomodoro_timer.start();
}





/*   OLD


fn main() {
    start_pomodoro(1, 1, 4);
}


fn start_timer (&self, minutes: i64, seconds: i64){
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
} */