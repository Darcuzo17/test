use std::f64::consts::PI;

pub struct LightAnalyzer;

#[derive(Debug)]
pub enum Error {
    InvalidLunarIdentifier,
    ModuleOverheat,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FasterMoon {
    FirstOne,
    SecondOne,
}

#[derive(Debug)]
pub struct LunarData {
    pub eclipse_time_difference: std::time::Duration,
    pub faster_moon: FasterMoon,
    pub orbital_period_avg: std::time::Duration,
}

impl LightAnalyzer {
    pub fn engage(moon1: &str, moon2: &str) -> Result<LunarData, Error> {
        let mut iter1 = moon1.split(Self::SPC);
        let _ = iter1.next().ok_or(Error::InvalidLunarIdentifier)?;
        let p1 = iter1
            .next()
            .ok_or(Error::InvalidLunarIdentifier)?
            .parse::<u64>()
            .map_err(|_| Error::InvalidLunarIdentifier)?;

        let mut iter2 = moon2.split(Self::SPC);
        let _ = iter2.next().ok_or(Error::InvalidLunarIdentifier)?;
        let p2 = iter2
            .next()
            .ok_or(Error::InvalidLunarIdentifier)?
            .parse::<u64>()
            .map_err(|_| Error::InvalidLunarIdentifier)?;

        if p1 == p2 {
            return Err(Error::ModuleOverheat);
        }

        let up = 2.0 * PI
            - iter1
                .next()
                .ok_or(Error::InvalidLunarIdentifier)?
                .parse::<f64>()
                .map_err(|_| Error::InvalidLunarIdentifier)?
                .to_radians()
            - iter2
                .next()
                .ok_or(Error::InvalidLunarIdentifier)?
                .parse::<f64>()
                .map_err(|_| Error::InvalidLunarIdentifier)?
                .to_radians();
        if up <= 0.0 || iter1.next().is_some() || iter2.next().is_some() {
            return Err(Error::ModuleOverheat);
        }

        Ok(LunarData {
            eclipse_time_difference: std::time::Duration::from_secs_f64(
                up / (2.0 * PI * (p1 as f64 - p2 as f64).abs() / (p1 as f64 * p2 as f64)),
            ),
            faster_moon: if p1 < p2 {
                FasterMoon::FirstOne
            } else {
                FasterMoon::SecondOne
            },
            orbital_period_avg: std::time::Duration::from_millis(500 * (p1 + p2)),
        })
    }

    const SPC: char = '\x23';
}
