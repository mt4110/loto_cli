mod oracle;
mod oracle_modules;
mod random;

use std::error::Error;
use std::fs::File;
use std::io::{self, IsTerminal, Write};
use std::time::Instant;

use clap::{CommandFactory, Parser, ValueEnum, error::ErrorKind};
use oracle::{AuraColor, BloodType};
use random::{Random, RandomKind};
use time::{
    Date, OffsetDateTime,
    macros::{format_description, offset},
};

#[derive(Parser, Debug)]
#[command(
    name = "loto-random-cli",
    version,
    about = "ロト6・ロト7の番号生成と乱数の実験"
)]
struct Cli {
    /// くじの種類
    #[arg(value_enum, default_value_t = GameType::Loto6)]
    game: GameType,

    /// pure（均等抽選）、oracle（占い）、odds（1等の確率）
    #[arg(default_value = "pure")]
    algorithm: String,

    /// 何口分生成するか
    #[arg(long, default_value_t = 10)]
    n: usize,

    /// CSVの出力先
    #[arg(long)]
    out: Option<String>,

    /// 乱数源。seeded は予測可能性を学ぶ教材専用
    #[arg(long, value_enum, default_value_t = RandomKind::Os)]
    rng: RandomKind,

    /// 教材モードの32ビットシード
    #[arg(long)]
    seed: Option<u32>,

    /// 生年月日 (YYYY-MM-DD)
    #[arg(long)]
    #[arg(value_parser = parse_birth_date)]
    birth_date: Option<Date>,

    /// 出生地の時計での出生時 (0–23)。四柱推命の時柱に使用
    #[arg(long, requires = "birth_date", value_parser = clap::value_parser!(u8).range(0..=23))]
    birth_hour: Option<u8>,

    /// 血液型（神託モードの追加要素）
    #[arg(long, value_enum)]
    blood_type: Option<BloodType>,

    /// オーラカラー（神託モードの追加要素）
    #[arg(long, value_enum)]
    aura_color: Option<AuraColor>,

    /// 神託モードで Enter 待ちを省略
    #[arg(long)]
    no_prompt: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum GameType {
    Loto6,
    Loto7,
}

impl GameType {
    fn config(self) -> (usize, usize) {
        match self {
            Self::Loto6 => (43, 6),
            Self::Loto7 => (37, 7),
        }
    }

    fn combinations(self) -> u64 {
        let (max, picks) = self.config();
        (1..=picks as u64).fold(1, |total, i| total * (max as u64 - i + 1) / i)
    }
}

fn invalid(message: &str) -> ! {
    Cli::command()
        .error(ErrorKind::InvalidValue, message)
        .exit()
}

fn parse_birth_date(value: &str) -> Result<Date, time::error::Parse> {
    Date::parse(value, format_description!("[year]-[month]-[day]"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let algorithm = cli.algorithm.to_ascii_lowercase();
    let oracle = matches!(algorithm.as_str(), "oracle" | "divine" | "destiny");
    let mut rng = match (cli.rng, cli.seed) {
        (RandomKind::Os, None) => Random::Os,
        (RandomKind::Seeded, Some(seed)) if !oracle => Random::Seeded(seed),
        (RandomKind::Seeded, _) if oracle => invalid("神託モードでは --rng os を指定してください"),
        (RandomKind::Seeded, None) => invalid("--rng seeded には --seed が必要です"),
        (RandomKind::Os, Some(_)) => invalid("--seed は --rng seeded 専用です"),
        _ => unreachable!(),
    };

    if algorithm == "odds" {
        let odds = cli.game.combinations();
        println!("1等の確率（1口） 1 / {odds}");
        println!(
            "毎週1口（年52口）の場合、初当せんまでの平均は約 {:.0} 年",
            odds as f64 / 52.0
        );
        return Ok(());
    }
    if matches!(rng, Random::Seeded(_)) {
        eprintln!("教材モード。同じシードから同じ番号列を再現できます。");
    }
    let (max, picks) = cli.game.config();
    let weights = if oracle {
        let observer = if !cli.no_prompt && io::stdin().is_terminal() && io::stderr().is_terminal()
        {
            eprint!("星の巡りを感じたら Enter を押してください… ");
            io::stderr().flush()?;
            let start = Instant::now();
            io::stdin().read_line(&mut String::new())?;
            Some(start.elapsed().as_nanos())
        } else {
            None
        };
        oracle::weights(
            max,
            OffsetDateTime::now_utc().to_offset(offset!(+9)).date(),
            cli.birth_date,
            cli.birth_hour,
            cli.blood_type,
            cli.aura_color,
            observer,
        )
    } else {
        vec![1; max]
    };

    let mut csv = cli.out.as_ref().map(File::create).transpose()?;
    if let Some(file) = &mut csv {
        write!(file, "draw")?;
        for i in 1..=picks {
            write!(file, ",n{i}")?;
        }
        writeln!(file)?;
    }

    let mut stdout = io::stdout().lock();
    for i in 1..=cli.n {
        let numbers = random::ticket(weights.clone(), picks, &mut rng)?;
        let line = numbers
            .iter()
            .map(|n| format!("{n:02}"))
            .collect::<Vec<_>>()
            .join(" , ");
        writeln!(stdout, "{line}")?;
        if let Some(file) = &mut csv {
            write!(file, "{i}")?;
            for n in numbers {
                write!(file, ",{n}")?;
            }
            writeln!(file)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odds_match_game_rules() {
        assert_eq!(GameType::Loto6.combinations(), 6_096_454);
        assert_eq!(GameType::Loto7.combinations(), 10_295_472);
    }
}
