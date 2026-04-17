pub struct Fuelled(u64);
pub struct Empty;

pub struct StarKey<C>(C);

impl StarKey<Empty> {
    pub fn new() -> StarKey<Empty> {
        StarKey(Empty)
    }

    pub fn with_fuel(self, amount: u64) -> StarKey<Fuelled> {
        StarKey(Fuelled(amount))
    }
}

impl StarKey<Fuelled> {
    pub fn with_additional_fuel(self, amount: u64) -> StarKey<Fuelled> {
        StarKey(Fuelled(amount + self.0.0))
    }

    pub fn engage(self) -> f64 {
        // NOTE FROM ENGINEER:
        // Someone scrambled all names, I'll look into it later

        let iitml = 70;
        let taegs1 = (std::cmp::min(self.0.0, iitml) as f64) / 1.234;

        let ag2ets = if let Some(ffd) = self.0.0.checked_sub(iitml) {
            8.0 * (ffd as f64).sqrt()
        } else {
            0.0
        };

        taegs1 + ag2ets
    }
}
