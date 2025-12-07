mod oracle;
mod oracle_modules;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::RangeInclusive;

use chrono::NaiveDate;
use clap::{Parser, ValueEnum};
use oracle::{AuraColor, BloodType, OracleContext, OracleEngine};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// CLI 引数定義
#[derive(Parser, Debug)]
#[command(
    name = "loto-random-cli",
    version,
    about = "ロト6 / ロト7 の完全ランダム数字ジェネレータ"
)]
struct Cli {
    /// 種類: loto6 or loto7
    #[arg(long, value_enum, default_value_t = GameType::Loto6)]
    r#type: GameType,

    /// アルゴリズム: pure, spread, cluster, favorite (hidden: oracle)
    #[arg(long, default_value = "pure")]
    algo: String,

    /// 何口分生成するか
    #[arg(long, default_value_t = 10)]
    n: usize,

    /// 出力CSVファイルパス（指定したときだけCSVに書き出す）
    #[arg(long)]
    out: Option<String>,

    // --- Oracle Mode Optionals ---
    /// 生年月日 (YYYY-MM-DD) - Oracle mode only
    #[arg(long)]
    birth_date: Option<NaiveDate>,

    /// 血液型 (A, B, O, AB) - Oracle mode only
    #[arg(long, value_enum)]
    blood_type: Option<BloodTypeArg>,

    /// オーラカラー - Oracle mode only
    #[arg(long, value_enum)]
    aura_color: Option<AuraColorArg>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum BloodTypeArg {
    A,
    B,
    O,
    AB,
}

impl From<BloodTypeArg> for BloodType {
    fn from(arg: BloodTypeArg) -> Self {
        match arg {
            BloodTypeArg::A => BloodType::A,
            BloodTypeArg::B => BloodType::B,
            BloodTypeArg::O => BloodType::O,
            BloodTypeArg::AB => BloodType::AB,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum AuraColorArg {
    Red,
    Blue,
    Green,
    Gold,
    Purple,
    White,
    Black,
}

impl From<AuraColorArg> for AuraColor {
    fn from(arg: AuraColorArg) -> Self {
        match arg {
            AuraColorArg::Red => AuraColor::Red,
            AuraColorArg::Blue => AuraColor::Blue,
            AuraColorArg::Green => AuraColor::Green,
            AuraColorArg::Gold => AuraColor::Gold,
            AuraColorArg::Purple => AuraColor::Purple,
            AuraColorArg::White => AuraColor::White,
            AuraColorArg::Black => AuraColor::Black,
        }
    }
}

/// ゲームタイプ
#[derive(Copy, Clone, Debug, ValueEnum)]
enum GameType {
    Loto6,
    Loto7,
}

impl GameType {
    /// 各ゲームの数値範囲と口あたりの個数を返す
    fn config(&self) -> (RangeInclusive<u32>, usize) {
        match self {
            GameType::Loto6 => (1..=43, 6),
            GameType::Loto7 => (1..=37, 7),
        }
    }
}

enum Algorithm {
    Pure,
    Spread,
    Cluster,
    Favorite,
    Oracle,
}

impl Algorithm {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pure" => Algorithm::Pure,
            "spread" => Algorithm::Spread,
            "cluster" => Algorithm::Cluster,
            "favorite" => Algorithm::Favorite,
            "oracle" | "divine" | "destiny" => {
                eprintln!("🔮 The forbidden Oracle mode has been invoked. Probability bends, but math remains unchanged.");
                Algorithm::Oracle
            }
            _ => Algorithm::Pure,
        }
    }
}

/// 1口分の番号を生成
fn generate_ticket(
    algo: &Algorithm,
    range: RangeInclusive<u32>,
    picks: usize,
    oracle_engine: &mut Option<OracleEngine>,
    oracle_ctx: &Option<OracleContext>,
) -> Vec<u32> {
    match algo {
        Algorithm::Oracle => {
            if let Some(engine) = oracle_engine {
                if let Some(ctx) = oracle_ctx {
                    return engine.divine(ctx);
                }
            }
            // Fallback if something went wrong
            pure_ticket(range, picks)
        }
        Algorithm::Pure | _ => pure_ticket(range, picks),
        // TODO: Implement other algos if needed, for now they fall back to pure or just placeholders
        // We focus on Oracle.
    }
}

fn pure_ticket(range: RangeInclusive<u32>, picks: usize) -> Vec<u32> {
    let mut nums: Vec<u32> = range.clone().collect();
    let mut rng = thread_rng();
    nums.shuffle(&mut rng);
    nums.truncate(picks);
    nums.sort();
    nums
}

/// CSVヘッダ行を作る: draw,n1,n2,...,n6/7
fn build_header(picks: usize) -> String {
    let mut s = String::from("draw");
    for i in 1..=picks {
        s.push_str(&format!(",n{}", i));
    }
    s.push('\n');
    s
}

/// 1行ぶんのCSV: 口番号 + 数字列
fn build_row(draw_index: usize, numbers: &[u32]) -> String {
    let mut s = format!("{}", draw_index);
    for n in numbers {
        s.push(',');
        s.push_str(&n.to_string());
    }
    s.push('\n');
    s
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let algo = Algorithm::from_str(&cli.algo);

    let (range, picks) = cli.r#type.config();
    let max = *range.end();

    // Prepare Oracle Engine if needed
    let mut oracle_ctx = None;
    let mut oracle_engine = None;

    if let Algorithm::Oracle = algo {
        // Mocking derived values for now; real implementation will come in modules
        let ctx = OracleContext::from_args(
            max,
            picks as u32,
            cli.birth_date,
            cli.blood_type.map(|b| b.into()),
            cli.aura_color.map(|a| a.into()),
        );
        oracle_ctx = Some(ctx);
    }

    // Correction: OracleContext holding ThreadRng is problematic if we want to build it here.
    // ThreadRng is thread-local handle.
    // We should probably just instantiate it inside divine() or pass it.
    // I will fix `oracle.rs` to NOT hold `rng: ThreadRng` in struct field if it causes issues,
    // but actually `rand::ThreadRng` is just a handle, maybe it's fine?
    // "ThreadRng does not implement Clone". So `rng.clone()` failed above.
    // I'll leave `rng` out of `OracleContext` in the fix, or init it inside.

    // For now, let's FIX oracle.rs before compiling main.rs fully?
    // OR I can just fix the `OracleContext` initialization below to not clone, but if I need it multiple times...
    // Actually, `divine` takes `&mut self` and `&OracleContext`.
    // The modules need randomness.
    // Best: `OracleContext` holds the seed or just allow modules to make their own `thread_rng()`.
    // Modules creating their own `thread_rng()` is fine and easier.

    // Let's remove `rng` from `OracleContext` in `oracle.rs` in next step.

    // out が指定されている場合だけ CSV を開く
    let mut csv_file = if let Some(path) = &cli.out {
        Some(File::create(path)?)
    } else {
        None
    };

    // CSV があればヘッダを書く
    if let Some(file) = csv_file.as_mut() {
        let header = build_header(picks);
        file.write_all(header.as_bytes())?;
    }

    // Init Engine
    if let Some(ctx) = &oracle_ctx {
        oracle_engine = Some(OracleEngine::new(ctx));
    }

    for i in 1..=cli.n {
        let ticket = generate_ticket(&algo, range.clone(), picks, &mut oracle_engine, &oracle_ctx);

        // 標準出力
        let line = ticket
            .iter()
            .map(|n| format!("{:02}", n))
            .collect::<Vec<_>>()
            .join(" , ");

        println!("{}", line);

        if let Some(file) = csv_file.as_mut() {
            let row = build_row(i, &ticket);
            file.write_all(row.as_bytes())?;
        }
    }

    Ok(())
}
