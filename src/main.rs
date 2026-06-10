use ask_input::input;

fn main() {
    let mut low = 1;
    let mut high = 100;
    let mut attempts = 0;

    println!("Загадай число от 1 до 100, а я угадаю!");
    println!("Нажми Enter, когда загадаешь...");
    let _: String = input().unwrap();

    while low <= high {
        let mid = (low + high) / 2;
        attempts += 1;

        println!("Твоё число {}?", mid);
        print!("Введи > (больше), < (меньше), = (угадал) или exit: ");

        let answer: String = input().unwrap();

        match answer.as_str() {
            "=" => {
                println!("Я угадал число {} за {} попыток!", mid, attempts);
                return;
            }
            ">" => low = mid + 1,
            "<" => high = mid - 1,
            "exit" | "quit" | "q" => {
                println!("Выход из игры. До свидания!");
                return;
            }
            _ => {
                println!("Ошибка: введи >, <, = или exit");
                attempts -= 1;
                continue;
            }
        }
    }
    println!("Жулик не воруй!");
}