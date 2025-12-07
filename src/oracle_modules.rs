use crate::oracle::{
    AuraColor, ChineseZodiac, DivinationModule, MoonPhase, OracleContext, Rokuyo, WesternZodiac,
};
use chrono::Datelike;

// --- 1. Western Astrology ---

pub struct WesternAstrology;

impl DivinationModule for WesternAstrology {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        if let Some(sign) = ctx.western_zodiac {
            eprintln!("[Astrology] Sign: {:?} derived from birth date.", sign);
            let range_len = weights.len() - 1; // 1-based

            match sign {
                WesternZodiac::Aries => {
                    eprintln!("            Favoring bold prime numbers & high ranges.");
                    for i in 1..=range_len {
                        // Boost high range
                        if i > (range_len * 7 / 10) {
                            weights[i] *= 1.2;
                        }
                        if is_prime(i as u32) {
                            weights[i] *= 1.3;
                        }
                    }
                }
                WesternZodiac::Taurus => {
                    eprintln!(
                        "            Favoring stability (numbers ending in 0, 5) and low range."
                    );
                    for i in 1..=range_len {
                        if i <= (range_len / 2) {
                            weights[i] *= 1.2;
                        }
                        if i % 5 == 0 {
                            weights[i] *= 1.3;
                        }
                    }
                }
                WesternZodiac::Gemini => {
                    eprintln!("            Favoring duality and communication (double digits).");
                    for i in 1..=range_len {
                        if i > 10 && i % 11 == 0 {
                            weights[i] *= 1.5;
                        }
                    }
                }
                WesternZodiac::Cancer => {
                    eprintln!("            Favoring numbers near the home (low range).");
                    for i in 1..=range_len {
                        if i <= (range_len / 3) {
                            weights[i] *= 1.3;
                        }
                    }
                }
                // Skip others for brevity, can enable generic logic
                _ => {
                    eprintln!("            Generic blessings for {:?}.", sign);
                }
            }
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// --- 2. Chinese Zodiac ---

pub struct ChineseZodiacModule;

impl DivinationModule for ChineseZodiacModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        if let Some(zodiac) = ctx.chinese_zodiac {
            eprintln!(
                "[Zodiac(Animal)] Year of the {:?} -> applying traits.",
                zodiac
            );
            let range_len = weights.len() - 1;

            match zodiac {
                ChineseZodiac::Dragon => {
                    eprintln!("               Empowering wide spread & large numbers.");
                    for i in 1..=range_len {
                        if i > range_len.saturating_sub(10) {
                            weights[i] *= 1.5;
                        }
                    }
                }
                ChineseZodiac::Rat => {
                    eprintln!("               Clever starts; boosting low numbers.");
                    for i in 1..=10 {
                        if i < weights.len() {
                            weights[i] *= 1.4;
                        }
                    }
                }
                ChineseZodiac::Tiger => {
                    eprintln!("               Aggressive power; boosting odds.");
                    for i in 1..=range_len {
                        if i % 2 != 0 {
                            weights[i] *= 1.2;
                        }
                    }
                }
                _ => {
                    eprintln!("               Standard fortune for this animal.");
                }
            }
        }
    }
}

// --- 3. Sanmei (Simplified) ---

pub struct SanmeiModule;

impl DivinationModule for SanmeiModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        // Derived from year if birth_date is present
        if let Some(date) = ctx.birth_date {
            let year = date.year();
            let stem = year % 10;

            let (element_name, boost_fn): (&str, fn(usize, &mut f64)) = match stem {
                4 | 5 => ("Wood", |i, w| {
                    if i % 3 == 0 {
                        *w *= 1.2
                    }
                }),
                6 | 7 => ("Fire", |i, w| {
                    if (i / 10 + i % 10) > 5 {
                        *w *= 1.2
                    }
                }),
                8 | 9 => ("Earth", |_, w| *w *= 1.05),
                0 | 1 => ("Metal", |i, w| {
                    if i % 2 == 0 {
                        *w *= 1.2
                    }
                }),
                2 | 3 => ("Water", |i, w| {
                    if i % 10 == 2 || i % 10 == 3 || i % 10 == 8 {
                        *w *= 1.2
                    }
                }),
                _ => ("Unknown", |_, _| {}),
            };

            eprintln!(
                "[Sanmei] Element: {} (Stem {}) -> biased weights.",
                element_name, stem
            );

            for i in 1..weights.len() {
                boost_fn(i, &mut weights[i]);
            }
        }
    }
}

// --- 4. Moon Phase ---

pub struct MoonPhaseModule;

impl DivinationModule for MoonPhaseModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        let range_len = weights.len() - 1;
        match ctx.moon_phase {
            MoonPhase::New => {
                eprintln!("[Moon] Phase: New -> favoring beginnings (low numbers).");
                for i in 1..=range_len / 2 {
                    weights[i] *= 1.2;
                }
            }
            MoonPhase::Waxing => {
                eprintln!("[Moon] Phase: Waxing -> favoring growth (ascending preference).");
                for i in 1..=range_len {
                    // Linear boost
                    let factor = 1.0 + (i as f64 / range_len as f64) * 0.3;
                    weights[i] *= factor;
                }
            }
            MoonPhase::Full => {
                eprintln!("[Moon] Phase: Full -> favoring abundance (even spread, high numbers).");
                for i in range_len / 2..=range_len {
                    weights[i] *= 1.25;
                }
            }
            MoonPhase::Waning => {
                eprintln!("[Moon] Phase: Waning -> favoring release (decending preference).");
                for i in 1..=range_len {
                    let factor = 1.3 - (i as f64 / range_len as f64) * 0.3;
                    weights[i] *= factor;
                }
            }
        }
    }
}

// --- 5. Rokuyo ---

pub struct RokuyoModule;

impl DivinationModule for RokuyoModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        match ctx.rokuyo {
            Rokuyo::Taian => {
                eprintln!("[Rokuyo] Taian (Great Peace) -> Even numbers gain a gentle blessing.");
                for i in 1..weights.len() {
                    if i % 2 == 0 {
                        weights[i] *= 1.15;
                    }
                }
            }
            Rokuyo::Butsumetsu => {
                eprintln!("[Rokuyo] Butsumetsu (Buddha's Death) -> Minimalistic patterns.");
                // Avoid extremes
                let len = weights.len();
                if len > 10 {
                    weights[1] *= 0.8;
                    weights[len - 1] *= 0.8;
                }
            }
            Rokuyo::Senkatsu => {
                eprintln!("[Rokuyo] Senkatsu (Win Early) -> Boosting first half.");
                let mid = weights.len() / 2;
                for i in 1..mid {
                    weights[i] *= 1.2;
                }
            }
            Rokuyo::Senbu => {
                eprintln!("[Rokuyo] Senbu (Lose Early, Win Late) -> Boosting second half.");
                let mid = weights.len() / 2;
                for i in mid..weights.len() {
                    weights[i] *= 1.2;
                }
            }
            _ => {
                eprintln!("[Rokuyo] {:?} -> General luck applied.", ctx.rokuyo);
            }
        }
    }
}

// --- 6. Feng Shui / Aura ---

pub struct FengShuiModule;

impl DivinationModule for FengShuiModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        if let Some(aura) = ctx.aura_color {
            // map aura to element/direction logic
            let range_len = weights.len() - 1;
            let quadrant_size = range_len / 4;

            // simple mapping
            match aura {
                AuraColor::Red => {
                    // Fire / South
                    eprintln!("[FengShui] Red Aura (South/Fire) -> Vitality in Q3.");
                    // Boost Q3
                    let start = quadrant_size * 2;
                    let end = quadrant_size * 3;
                    for i in start..end {
                        if i < weights.len() {
                            weights[i] *= 1.3;
                        }
                    }
                }
                AuraColor::Gold => {
                    // Metal / West
                    eprintln!("[FengShui] Gold Aura (West/Metal) -> Wealth in Q4.");
                    let start = quadrant_size * 3;
                    for i in start..weights.len() {
                        weights[i] *= 1.3;
                    }
                }
                AuraColor::Green => {
                    // Wood / East
                    eprintln!("[FengShui] Green Aura (East/Wood) -> Growth in Q1.");
                    for i in 1..quadrant_size {
                        weights[i] *= 1.3;
                    }
                }
                AuraColor::Blue => {
                    // Water / North
                    eprintln!("[FengShui] Blue Aura (North/Water) -> Flow in Q2.");
                    let start = quadrant_size;
                    let end = quadrant_size * 2;
                    for i in start..end {
                        if i < weights.len() {
                            weights[i] *= 1.3;
                        }
                    }
                }
                _ => {
                    eprintln!("[FengShui] {:?} Aura -> Harmonizing all quadrants.", aura);
                    // slight global boost or noise
                }
            }
        }
    }
}

// --- 7. Blood Type ---

pub struct BloodTypeModule;

impl DivinationModule for BloodTypeModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        if let Some(bt) = ctx.blood_type {
            eprintln!("[BloodType] Type {:?} -> applying biological bias.", bt);
            let range_len = weights.len() - 1;

            match bt {
                crate::oracle::BloodType::A => {
                    eprintln!("            Favoring balanced gaps and moderate sums.");
                    // Boost middle 1/3
                    let start = range_len / 3;
                    let end = range_len * 2 / 3;
                    for i in start..=end {
                        weights[i] *= 1.25;
                    }
                }
                crate::oracle::BloodType::B => {
                    eprintln!("            Favoring individuality (unusual numbers).");
                    // Boost primes? or ends
                    for i in 1..=range_len {
                        if i < 5 || i > range_len - 5 {
                            weights[i] *= 1.3;
                        }
                    }
                }
                crate::oracle::BloodType::O => {
                    eprintln!("            Favoring broad ranges and big numbers.");
                    // Boost upper 50%
                    for i in range_len / 2..=range_len {
                        weights[i] *= 1.2;
                    }
                }
                crate::oracle::BloodType::AB => {
                    eprintln!("            Favoring symmetrical patterns.");
                    // Boost numbers with double digits e.g. 11, 22, 33 OR sums
                    for i in 1..=range_len {
                        if i > 9 && i % 11 == 0 {
                            weights[i] *= 1.5;
                        }
                    }
                }
            }
        }
    }
}

// --- 8. Chaos / Entropy ---

pub struct ChaosModule;

impl DivinationModule for ChaosModule {
    fn apply(&self, _ctx: &OracleContext, weights: &mut [f64]) {
        // Use memory address or time for chaos
        let p = weights.as_ptr() as usize;
        let t = chrono::Utc::now().timestamp_subsec_nanos();
        let seed = (p as u64) ^ (t as u64);

        eprintln!(
            "[Chaos] Tortoise shell cracks along unseen lines (entropy: 0x{:X}...).",
            seed
        );

        // Pseudo-random perturbation without changing rng state of main context
        // We use a simple hash to deterministicly noise it up based on 'seed' + index

        for i in 1..weights.len() {
            let mut x = (seed ^ (i as u64)).wrapping_mul(0x517cc1b727220a95);
            x ^= x >> 12; // PCG-ish step
            let noise = (x % 100) as f64 / 1000.0; // 0.00 .. 0.09

            weights[i] += noise;
        }
    }
}

// --- 9. Stats / Hot-Cold (Mock) ---

pub struct StatsModule;

impl DivinationModule for StatsModule {
    fn apply(&self, ctx: &OracleContext, weights: &mut [f64]) {
        // Mocking Akashic Records
        eprintln!("[Stats] Historical resonance -> boosting numbers that echo the past.");

        // Pretend we have stats.
        // Let's say we favor numbers that match 'current day' or 'month' as hot numbers
        // and suppress numbers that match 'hour'

        let day = ctx.now_utc.day();
        let month = ctx.now_utc.month();

        for i in 1..weights.len() {
            let n = i as u32;
            if n == day || n == month || n == (day + month) {
                weights[i] *= 1.5; // HOT
            }
        }
    }
}
