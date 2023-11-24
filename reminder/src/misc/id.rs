use std::{fmt::Display, sync::Mutex};

use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use rand::Rng;

const BASE36_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
const TIME_2000: Lazy<DateTime<Utc>> = Lazy::new(|| {
    DateTime::parse_from_str("2000/1/1 00:00:00.000 +0000", "%Y/%m/%d %H:%M:%S%.3f %z")
        .unwrap()
        .with_timezone(&Utc)
});
static RANDOM_COUNTER_SEED: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(rand::thread_rng().gen()));

#[derive(Debug, Clone)]
pub struct Id(Box<[u8; 10]>);

impl Id {
    pub fn new() -> Self {
        let duration = Utc::now() - TIME_2000.clone();
        let duration_millisecond = if duration.ge(&Duration::milliseconds(0)) {
            duration.num_milliseconds()
        } else {
            0
        };

        let encoded_duration = encode_base36(duration_millisecond);
        let random_bytes = encode_base36(random_counter().into());
        let random_length = random_bytes.len();
        let random_bytes = random_bytes[random_length - 2..random_length].to_vec();

        let id: [u8; 10] = [encoded_duration, random_bytes]
            .concat()
            .try_into()
            .unwrap_or_else(|v| panic!("Could not create new id with {:?}", v));

        Id(Box::new(id))
    }

    pub fn parse(self) -> DateTime<Utc> {
        let encoded_duration = &self.0[..8];

        let duration_millisecond = encoded_duration
            .iter()
            .fold(0, |a: i64, &b| a * 36 + b as i64);

        TIME_2000.clone() + Duration::milliseconds(duration_millisecond)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|&c| BASE36_CHARS[c as usize] as char)
                .collect::<String>()
        )?;
        Ok(())
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        let mut result: [u8; 10] = [0; 10];
        let binding = value.to_ascii_lowercase();
        let chars = binding.as_bytes();

        for i in 0..10 {
            result[i] = BASE36_CHARS.iter().position(|&c| c == chars[i]).unwrap() as u8;
        }

        Self(Box::new(result))
    }
}

fn random_counter() -> u16 {
    let mut counter = RANDOM_COUNTER_SEED.lock().unwrap();
    *counter += 1;

    counter.clone()
}

fn encode_base36(mut num: i64) -> Vec<u8> {
    let mut result = vec![];

    while num > 0 {
        let rem = (num % 36) as u8;
        result.push(rem);

        num /= 36;
    }
    result.reverse();

    result
}

#[cfg(test)]
mod test {
    use chrono_tz::Asia;

    use super::*;

    #[test]
    fn generate_new_id() {
        let id = Id::new();

        println!("{}", id.to_string())
    }

    #[test]
    fn parse_datetime_from_id() {
        let id = Id::new();

        println!("{:?}", id.parse().with_timezone(&Asia::Tokyo))
    }

    #[test]
    fn parse_id_from_string() {
        let str = "9ldtw7e4l7".to_string();

        println!("{}", Id::from(str))
    }
}
