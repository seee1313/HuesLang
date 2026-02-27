# HuesLang

Системный язык программирования с переключаемой строгостью.

## Режимы

!#script-kiddy  // Всё прощает
!#normal        // Баланс
!#strict        // Строгий режим
!#unsafe        // Разрешает сырые указатели
!#no-os         // Для ядер и embedded

## Типы

int       // Целые числа
float     // Дробные числа
bool      // true / false
string    // Текст

## Переменные

hui x = 42           // Неизменяемая
piz y = 10           // Изменяемая
const PI = 3.14      // Константа

## Функции

hue greet(name: string) {
    prt("Привет, " + name)
}

hue add(a: int, b: int) -> int {
    ret a + b
}

## Структуры

construct Player {
    Health: int,
    Alive: bool
}

hui player = Player {
    Health: 100,
    Alive: true
}

## Циклы

// Бесконечный цикл
zalyping {
    prt("Навечно")
}

// While цикл
poka x < 10 {
    prt(x)
    x = x + 1
}

// For цикл
for i in 0..10 {
    prt(i)
}

## Условия

check age < 18 {
    prt("Пошёл нахуй")
}
then {
    prt("Проходи")
}

## Импорт

give @std/io
give @std/math

## Пример программы

!#normal

give @std/io

hue main() {
    hui name = "Мир"
    prt("Привет, " + name)
}
