use time::{Date, macros::date};

const STEMS: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
const BRANCHES: [char; 12] = [
    '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥',
];

#[derive(Debug, PartialEq, Eq)]
pub struct Pillar {
    pub stem: usize,
    pub branch: usize,
}

impl std::fmt::Display for Pillar {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "{}{}", STEMS[self.stem], BRANCHES[self.branch])
    }
}

pub fn pillars(date: Date, hour: Option<u8>) -> Vec<Pillar> {
    // Fixed civil dates approximate the twelve solar-term boundaries.
    const MONTH_BOUNDARIES: [u8; 12] = [6, 4, 6, 5, 6, 6, 7, 8, 8, 8, 7, 7];
    let month = date.month() as usize - 1;
    let before_boundary = i32::from(date.day() < MONTH_BOUNDARIES[month]);
    let month_index = (month as i32 - 1 - before_boundary).rem_euclid(12) as usize;
    let year = date.year() - i32::from((date.month() as u8, date.day()) < (2, 4));
    let year_cycle = (year - 4).rem_euclid(60) as usize;

    // The civil day cycle uses midnight and the Gregorian calendar.
    let day_cycle = (date.to_julian_day() + 49).rem_euclid(60) as usize;
    let mut result = vec![
        Pillar {
            stem: year_cycle % 10,
            branch: year_cycle % 12,
        },
        Pillar {
            stem: (year_cycle % 5 * 2 + 2 + month_index) % 10,
            branch: (month_index + 2) % 12,
        },
        Pillar {
            stem: day_cycle % 10,
            branch: day_cycle % 12,
        },
    ];
    if let Some(hour) = hour {
        let branch = usize::from(hour).div_ceil(2) % 12;
        result.push(Pillar {
            stem: (day_cycle % 5 * 2 + branch) % 10,
            branch,
        });
    }
    result
}

pub fn western_sign(date: Date) -> (usize, &'static str) {
    const SIGNS: [&str; 12] = [
        "山羊座",
        "水瓶座",
        "魚座",
        "牡羊座",
        "牡牛座",
        "双子座",
        "蟹座",
        "獅子座",
        "乙女座",
        "天秤座",
        "蠍座",
        "射手座",
    ];
    const BOUNDARIES: [u8; 12] = [20, 19, 21, 20, 21, 22, 23, 23, 23, 24, 23, 22];
    let month = date.month() as usize - 1;
    let index = (month + usize::from(date.day() >= BOUNDARIES[month])) % 12;
    (index, SIGNS[index])
}

pub fn moon_phase(date: Date) -> (usize, &'static str) {
    const PHASES: [&str; 4] = ["新月", "上弦", "満月", "下弦"];
    let reference = date!(2000 - 01 - 06);
    let days = (date.to_julian_day() - reference.to_julian_day()) as f64;
    let index = (4.0 * days.rem_euclid(29.53059) / 29.53059).round() as usize % 4;
    (index, PHASES[index])
}

pub fn digital_root(n: u32) -> u32 {
    1 + (n - 1) % 9
}

pub fn numerology(date: Date) -> u32 {
    let digits = date.year().unsigned_abs() * 10_000
        + u32::from(date.month() as u8) * 100
        + u32::from(date.day());
    digital_root(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pillars_cross_the_approximate_solar_year_boundary() {
        let date = date!(2024 - 02 - 03);
        let before = pillars(date, None);
        let after = pillars(date.next_day().unwrap(), Some(23));
        assert_eq!(before[0].to_string(), "癸卯");
        assert_eq!(before[1].to_string(), "乙丑");
        assert_eq!(after[0].to_string(), "甲辰");
        assert_eq!(after[1].to_string(), "丙寅");
        assert_eq!(before.len(), 3);
        assert_eq!(after.len(), 4);
        assert_eq!(after[3].branch, 0);
        assert_eq!(after[3].stem, after[2].stem % 5 * 2);
    }

    #[test]
    fn day_cycle_matches_reference_and_numerology_reduces_digits() {
        let date = date!(2024 - 02 - 10);
        assert_eq!(pillars(date, None)[2].to_string(), "甲辰");
        assert_eq!(numerology(date), 2);
        assert_eq!(western_sign(date).1, "水瓶座");
    }
}
