use sysinfo::{System, SystemExt};
use whoami::{username, fallible::hostname};
use chrono::{Local, DateTime};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Получение имени компьютера и пользователя
    println!("Имя компьютера: {}", hostname().unwrap_or_else(|_| "Unknown".to_string()));
    println!("Имя пользователя: {}", username());

    // Получение версии операционной системы
    let system = System::new_all();
    println!("Версия ОС: {}", system.os_version().unwrap_or_else(|| "Error".to_string()));

    // Получение системных метрик
    println!("Количество ядер CPU: {}", system.cpus().len());
    println!("Общий объем памяти: {} MB", system.total_memory() / 1024);
    println!("Используемая память: {} MB", system.used_memory() / 1024);

    // Функции для работы со временем
    let now: DateTime<Local> = Local::now();
    println!("Текущее локальное время: {}", now.format("%Y-%m-%d %H:%M:%S"));

    let system_time = SystemTime::now();
    let since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    println!("Время с начала эпохи UNIX: {} секунд", since_epoch.as_secs());

    // Дополнительные API-функции
    println!("Свободная память: {} MB", system.free_memory() / 1024);
    println!("Общий объем swap: {} MB", system.total_swap() / 1024);
    println!("Используемый swap: {} MB", system.used_swap() / 1024);
    println!("Количество процессов: {}", system.processes().len());
}