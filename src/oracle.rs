use chrono::{DateTime, NaiveDate, Utc};
// use rand::rngs::ThreadRng; // unused
// use std::collections::HashMap; // unused

// --- Enums for user input ---
#[derive(Debug, Clone, Copy)]
pub enum BloodType {
    A,
    B,
    O,
    AB,
}

#[derive(Debug, Clone, Copy)]
pub enum AuraColor {
    Red,
    Blue,
    Green,
    Gold,
    Purple,
    White,
    Black,
}

pub use crate::oracle_modules::*;

// --- Derived Astrological Enums ---

#[derive(Debug, Clone, Copy)]
pub enum WesternZodiac {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChineseZodiac {
    Rat,
    Ox,
    Tiger,
    Rabbit,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Rooster,
    Dog,
    Pig,
}

#[derive(Debug, Clone, Copy)]
pub enum MoonPhase {
    New,
    Waxing,
    Full,
    Waning,
}

#[derive(Debug, Clone, Copy)]
pub enum Rokuyo {
    Taian,
    Butsumetsu,
    Tomobiki,
    Senkatsu,
    Senbu,
    Shakku,
}

#[derive(Debug, Clone, Copy)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

// --- Context and Engine ---
#[derive(Debug)]
#[allow(dead_code)]
pub struct OracleContext {
    pub max: u32,
    pub count: u32,
    pub now_utc: DateTime<Utc>,

    // User inputs
    pub birth_date: Option<NaiveDate>,
    pub blood_type: Option<BloodType>,
    pub aura_color: Option<AuraColor>,

    // System
    // rng is removed, modules should instantiate thread_rng() themselves or we pass it in methods
    pub host_fingerprint: u64,
    pub system_load: Option<f32>, // Memory usage percentage (0.0 - 100.0)
    pub observer_resonance: Option<u128>, // Nanoseconds resonance

    // Derived (computed in new())
    pub western_zodiac: Option<WesternZodiac>,
    pub chinese_zodiac: Option<ChineseZodiac>,
    pub rokuyo: Rokuyo,
    pub moon_phase: MoonPhase,
    pub weekday: Weekday,
}

impl OracleContext {
    pub fn from_args(
        max: u32,
        count: u32,
        birth_date: Option<NaiveDate>,
        blood_type: Option<BloodType>,
        aura_color: Option<AuraColor>,
    ) -> Self {
        use chrono::Datelike;
        use std::io::{self, Write};
        use std::time::{Instant, SystemTime, UNIX_EPOCH};
        use sysinfo::System;

        let now_utc = Utc::now();

        // --- 1. Digital Animism (Machine Spirit) ---
        let mut sys = System::new_all();
        sys.refresh_all();
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let system_load = if total_mem > 0 {
            Some((used_mem as f32 / total_mem as f32) * 100.0)
        } else {
            None
        };

        // --- 2. Quantum Observer Effect ---
        eprintln!("🌌 Awaiting Observer Intervention...");
        eprint!("   Press [ENTER] when you feel the cosmic alignment: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let start = Instant::now();
        io::stdin().read_line(&mut input).unwrap();
        
        let elapsed = start.elapsed().as_nanos();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let resonance = elapsed ^ timestamp; // XOR mixing
        
        eprintln!("⚡ Quantum state collapsed at {}ns. Resonance: {:x}", elapsed, resonance);

        // Derivations
        let western_zodiac = birth_date.map(derive_western_zodiac);
        let chinese_zodiac = birth_date.map(|d| derive_chinese_zodiac(d.year()));
        let rokuyo = derive_rokuyo(now_utc);
        let moon_phase = derive_moon_phase(now_utc);
        
        // Weekday
        let weekday = match now_utc.weekday() {
            chrono::Weekday::Mon => Weekday::Mon,
            chrono::Weekday::Tue => Weekday::Tue,
            chrono::Weekday::Wed => Weekday::Wed,
            chrono::Weekday::Thu => Weekday::Thu,
            chrono::Weekday::Fri => Weekday::Fri,
            chrono::Weekday::Sat => Weekday::Sat,
            chrono::Weekday::Sun => Weekday::Sun,
        };

        // Pseudo fingerprint mixed with resonance
        let host_fingerprint = 0xCAFEBABE ^ (resonance as u64); 

        OracleContext {
            max,
            count,
            now_utc,
            birth_date,
            blood_type,
            aura_color,
            host_fingerprint,
            system_load,
            observer_resonance: Some(resonance),
            western_zodiac,
            chinese_zodiac,
            rokuyo,
            moon_phase,
            weekday,
        }
    }
}

// Helpers
fn derive_western_zodiac(d: NaiveDate) -> WesternZodiac {
    use chrono::Datelike;
    let day = d.day();
    match d.month() {
        3 => {
            if day >= 21 {
                WesternZodiac::Aries
            } else {
                WesternZodiac::Pisces
            }
        }
        4 => {
            if day >= 20 {
                WesternZodiac::Taurus
            } else {
                WesternZodiac::Aries
            }
        }
        5 => {
            if day >= 21 {
                WesternZodiac::Gemini
            } else {
                WesternZodiac::Taurus
            }
        }
        6 => {
            if day >= 22 {
                WesternZodiac::Cancer
            } else {
                WesternZodiac::Gemini
            }
        }
        7 => {
            if day >= 23 {
                WesternZodiac::Leo
            } else {
                WesternZodiac::Cancer
            }
        }
        8 => {
            if day >= 23 {
                WesternZodiac::Virgo
            } else {
                WesternZodiac::Leo
            }
        }
        9 => {
            if day >= 23 {
                WesternZodiac::Libra
            } else {
                WesternZodiac::Virgo
            }
        }
        10 => {
            if day >= 24 {
                WesternZodiac::Scorpio
            } else {
                WesternZodiac::Libra
            }
        }
        11 => {
            if day >= 23 {
                WesternZodiac::Sagittarius
            } else {
                WesternZodiac::Scorpio
            }
        }
        12 => {
            if day >= 22 {
                WesternZodiac::Capricorn
            } else {
                WesternZodiac::Sagittarius
            }
        }
        1 => {
            if day >= 20 {
                WesternZodiac::Aquarius
            } else {
                WesternZodiac::Capricorn
            }
        }
        2 => {
            if day >= 19 {
                WesternZodiac::Pisces
            } else {
                WesternZodiac::Aquarius
            }
        }
        _ => WesternZodiac::Aries, // fallback
    }
}

fn derive_chinese_zodiac(year: i32) -> ChineseZodiac {
    match (year - 4) % 12 {
        0 => ChineseZodiac::Rat,
        1 => ChineseZodiac::Ox,
        2 => ChineseZodiac::Tiger,
        3 => ChineseZodiac::Rabbit,
        4 => ChineseZodiac::Dragon,
        5 => ChineseZodiac::Snake,
        6 => ChineseZodiac::Horse,
        7 => ChineseZodiac::Goat,
        8 => ChineseZodiac::Monkey,
        9 => ChineseZodiac::Rooster,
        10 => ChineseZodiac::Dog,
        _ => ChineseZodiac::Pig,
    }
}

fn derive_rokuyo(now: DateTime<Utc>) -> Rokuyo {
    use chrono::Datelike;
    // Mock: just use day of month
    match now.day() % 6 {
        0 => Rokuyo::Taian,
        1 => Rokuyo::Butsumetsu,
        2 => Rokuyo::Tomobiki,
        3 => Rokuyo::Senkatsu,
        4 => Rokuyo::Senbu,
        _ => Rokuyo::Shakku,
    }
}

fn derive_moon_phase(now: DateTime<Utc>) -> MoonPhase {
    use chrono::Datelike;
    // Mock: sync with 30 day cycle roughly
    let day = now.day() % 30; // 0..29
    if day < 7 {
        MoonPhase::New
    } else if day < 15 {
        MoonPhase::Waxing
    } else if day < 22 {
        MoonPhase::Full
    } else {
        MoonPhase::Waning
    }
}

pub trait DivinationModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]);
}

pub struct OracleEngine {
    modules: Vec<Box<dyn DivinationModule>>,
}

impl OracleEngine {
    pub fn new(_ctx: &OracleContext) -> Self {
        let mut modules: Vec<Box<dyn DivinationModule>> = vec![];

        // Register modules
        // 1. Western Astrology
        modules.push(Box::new(WesternAstrology));
        // 2. Chinese Zodiac
        modules.push(Box::new(ChineseZodiacModule));
        // 3. Sanmei
        modules.push(Box::new(SanmeiModule));
        // 4. Moon Phase
        modules.push(Box::new(MoonPhaseModule));
        // 5. Rokuyo
        modules.push(Box::new(RokuyoModule));
        // 6. Feng Shui
        modules.push(Box::new(FengShuiModule));
        // 7. Blood Type
        modules.push(Box::new(BloodTypeModule));
        // 8. Chaos
        modules.push(Box::new(ChaosModule));
        // 9. Stats
        modules.push(Box::new(StatsModule));

        // Return engine
        Self { modules }
    }

    pub fn divine(&mut self, ctx: &OracleContext) -> Vec<u32> {
        let range_len = ctx.max as usize;
        let mut weights = vec![1.0; range_len + 1]; // 1-based index (0 unused)

        eprintln!("🔮 THE ORACLE ENGAGES (神託起動)");
        eprintln!("----------------------------------------");

        for module in &self.modules {
            module.apply(ctx, &mut weights);
        }

        eprintln!("----------------------------------------");
        eprintln!("🌌 Converging timelines (世界線収束)...");

        // Normalize
        let sum: f64 = weights.iter().skip(1).sum();
        let mean = sum / (range_len as f64);
        if mean > 0.0 {
            for w in weights.iter_mut() {
                *w /= mean;
            }
        }

        // Weighted sampling
        use rand::distributions::WeightedIndex;
        use rand::prelude::*;

        // Build WeightedIndex from weights[1..] (since 0 is unused)
        // We need to map indices back to numbers 1..=max
        let valid_weights: Vec<f64> = weights.into_iter().skip(1).collect();

        let mut result = Vec::new();
        let mut rng = rand::thread_rng();

        // Simple weighted sampling without replacement
        // Note: WeightedIndex is immutable, so for without-replacement we might need
        // to reject duplicates or re-build distribution.
        // For small 'count', rejection sampling (check if exists) is fine.

        let dist = WeightedIndex::new(&valid_weights).unwrap();

        while result.len() < ctx.count as usize {
            let idx = dist.sample(&mut rng);
            let number = (idx + 1) as u32; // 0-index -> 1-based number

            if !result.contains(&number) {
                result.push(number);
            }
        }

        result.sort();
        eprintln!(
            "✨ REVELATION (啓示): [{}]",
            result
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        eprintln!("(Disclaimer: This is still just biased randomness. The universe laughs in expected value.)");

        result
    }
}
