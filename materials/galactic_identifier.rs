pub struct GalacticIdentifier;

#[derive(Debug)]
pub enum Error {
    InvalidStarIdentifier,
    ModuleOverheat,
}

impl GalacticIdentifier {
    pub fn engage(star1: &str, star2: &str) -> Result<Option<f64>, Error> {
        let mut iter1 = star1.split(Self::SPC);
        let mut iter2 = star2.split(Self::SPC);
        Ok(
            if iter1
                .next()
                .ok_or(Error::InvalidStarIdentifier)?
                .chars()
                .filter(|c| !c.is_ascii_digit())
                .collect::<Box<str>>()
                == iter2
                    .next()
                    .ok_or(Error::InvalidStarIdentifier)?
                    .chars()
                    .filter(|c| !c.is_ascii_digit())
                    .collect::<Box<str>>()
            {
                Some(
                    iter1
                        .zip(iter2)
                        .try_fold(0f64, |mut acc, (el1, el2)| {
                            acc += (el1
                                .parse::<f64>()
                                .map_err(|_| Error::InvalidStarIdentifier)?
                                - el2
                                    .parse::<f64>()
                                    .map_err(|_| Error::InvalidStarIdentifier)?)
                            .powi(2);
                            Ok(acc)
                        })?
                        .sqrt(),
                )
            } else {
                None
            },
        )
    }

    const SPC: char = '\x23';
}
