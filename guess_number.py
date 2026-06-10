def main():
    low, high = 1, 100
    attempts = 0

    print("Загадай число от 1 до 100, а я угадаю!")
    input("Нажми Enter, когда загадаешь...")

    while low <= high:
        mid = (low + high) // 2
        attempts += 1

        print(f"Твоё число {mid}?")
        answer = input("Введи > (больше), < (меньше), = (угадал) или q (выход): ").strip().lower()

        if answer == "=":
            print(f"Я угадал число {mid} за {attempts} попыток!")
            return
        elif answer == ">":
            low = mid + 1
        elif answer == "<":
            high = mid - 1
        elif answer == "q" or answer == "выход":
            print("Выход из игры. До встречи!")
            return
        else:
            print("Ошибка: введи >, <, = или q")
            attempts -= 1
            continue
    print("Жулик не воруй!")


if __name__ == "__main__":
    main()
