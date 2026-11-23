use clap::ValueEnum;
use time::Date;

use crate::oracle_modules::{digital_root, moon_phase, numerology, pillars, western_sign};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BloodType {
    A,
    B,
    O,
    AB,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum AuraColor {
    Red,
    Blue,
    Green,
    Gold,
    Purple,
    White,
    Black,
}

pub fn weights(
    max: usize,
    today: Date,
    birth_date: Option<Date>,
    birth_hour: Option<u8>,
    blood: Option<BloodType>,
    aura: Option<AuraColor>,
    observer: Option<u128>,
) -> Vec<u32> {
    let date = birth_date.unwrap_or(today);
    let (phase, phase_name) = moon_phase(today);
    let (sign, sign_name) = western_sign(date);
    let pillars = pillars(date, birth_hour);
    let year_branch = pillars[0].branch;
    let life_number = numerology(date);

    eprintln!("🔮 神託モード。数字選びの遊びです。当せん確率は変わりません。");
    eprintln!("  占星術（月相の概算） {phase_name}");
    eprintln!(
        "  四柱推命（簡易） {}",
        pillars
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("・")
    );
    eprintln!("  東洋占い（十二支・色） {} / {aura:?}", pillars[0]);
    eprintln!("  数字占い 運命数 {life_number}");
    eprintln!("  西洋占い（太陽星座） {sign_name}");
    if birth_date.is_none() {
        eprintln!("  生年月日の代わりに今日の日付を使用");
    }
    if birth_hour.is_none() {
        eprintln!("  出生時刻の指定がないため時柱を省略");
    }

    (1..=max)
        .map(|n| {
            let mut weight = 10;
            weight += 3 * u32::from((n - 1) * 4 / max == phase);
            weight += pillars
                .iter()
                .map(|pillar| {
                    2 * u32::from((n - 1) % 5 == pillar.stem / 2)
                        + u32::from((n - 1) % 12 == pillar.branch)
                })
                .sum::<u32>();
            weight += 3 * u32::from(
                (n - 1 + today.weekday().number_days_from_monday() as usize) % 12 == year_branch,
            );
            weight += 4 * u32::from(digital_root(n as u32) == life_number);
            weight += 3 * u32::from((n - 1) % 4 == sign % 4);
            if let Some(color) = aura {
                weight += 3 * u32::from((n - 1) % 7 == color as usize);
            }
            if let Some(kind) = blood {
                weight += 2 * u32::from((n - 1) % 4 == kind as usize);
            }
            if let Some(timing) = observer {
                weight += 2 * u32::from((n - 1) % 5 == (timing % 5) as usize);
            }
            weight
        })
        .collect()
}
