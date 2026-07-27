use ask_input::input;

const MIN: i32 = 1;
const MAX: i32 = 100;

fn main() {
    let mut low = 1;
    let mut high = 100;
    let mut attempts = 0;

    print_start_message();
    wait_for_enter();

    while low <= high {
        let mid = calculate_mid(low, high);

        print_guess_message(mid);
        let answer = get_user_answer();

        if process_answer(&answer, mid, &mut low, &mut high, &mut attempts) {
            return;
        }
    }
    
    print_cheater_message();
}

fn print_start_message() {
    println!("Загадай число от 1 до 100, а я угадаю!");
}

fn wait_for_enter() {
    println!("Нажми Enter, когда загадаешь...");
    let _: String = input().unwrap();
}

fn calculate_mid(low: i32, high: i32) -> i32 {
    (low + high) / 2
}

fn print_guess_message(mid: i32) {
    println!("Твоё число {}?", mid);
}

fn get_user_answer() -> String {
    print!("Введи > (больше), < (меньше), = (угадал) или exit: ");
    input().unwrap()
}

fn process_answer(answer: &str, mid: i32, low: &mut i32, high: &mut i32, attempts: &mut i32) -> bool {
    match answer {
        "=" => {
            println!("Я угадал число {} за {} попыток!", mid, *attempts + 1);
            true
        }
        ">" => {
            *attempts += 1;
            *low = mid + 1;
            false
        }
        "<" => {
            *attempts += 1;
            *high = mid - 1;
            false
        }
        "exit" | "quit" | "q" => {
            println!("Выход из игры. До свидания!");
            true
        }
        _ => {
            println!("Ошибка: введи >, <, = или exit");
            false
        }
    }
}

fn print_cheater_message() {
    println!("Жулик не воруй!");
}
