use clap::{Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Минимальная длина акронима для алиаса
    #[arg(short = 'l', long, default_value_t = 1)]
    min: usize,

    /// Количество предлагаемых акронимов
    #[arg(short, long, default_value_t = 20)]
    count: usize,

    /// Фильтровать алиасы, уже используемые в истории
    #[arg(long, default_value_t = false)]
    filter: bool,

    /// Использовать историю zsh вместо bash
    #[arg(short, long, default_value_t = false)]
    zsh: bool,

    /// Свой путь к папке с .bash_aliases/.bash_history/.zsh_history
    #[arg(short = 'f', long)]
    path: Option<PathBuf>,

    /// Включить режим отладки
    #[arg(long, default_value_t = false)]
    debug: bool,
}

pub fn run() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode включен: {:?}", args);
    }

    // TODO: Передать параметры в aliascore::run, реализовать основную логику
    println!(
        "min: {}, count: {}, filter: {}, zsh: {}, debug: {}, path: {:?}",
        args.min, args.count, args.filter, args.zsh, args.debug, args.path
    );
}