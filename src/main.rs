use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    cursor::MoveToColumn
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();

    // Устанавливаем белый цвет для рамки
    execute!(stdout, SetForegroundColor(Color::White)).unwrap();

    // Выводим верхнюю часть рамки
    execute!(stdout, Print("╔")).unwrap();
    for _ in 0..5 {
        execute!(stdout, Print("═")).unwrap();
    }

    // Устанавливаем красный цвет для текста
    execute!(stdout, SetForegroundColor(Color::Red)).unwrap();
    execute!(stdout, Print("Alien Invasion")).unwrap();

    execute!(stdout, SetForegroundColor(Color::White)).unwrap();
    for _ in 0..39 {
        execute!(stdout, Print("═")).unwrap();
    }
    execute!(stdout, Print("╗\n")).unwrap();

    for _ in 0..40 {
        execute!(stdout, Print("║")).unwrap();
        execute!(stdout, MoveToColumn(59)).unwrap();
        execute!(stdout, Print("║\n")).unwrap();
    }

    execute!(stdout, Print("╚")).unwrap();
    for _ in 0..58 {
        execute!(stdout, Print("═")).unwrap();
    }
    execute!(stdout, Print("╝\n")).unwrap();

    // Сброс цвета
    execute!(stdout, ResetColor).unwrap();

    // Сбрасываем буфер
    stdout.flush().unwrap();
}
