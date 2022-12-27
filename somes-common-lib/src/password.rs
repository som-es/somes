use dotenv_codegen::dotenv;
use once_cell::sync::Lazy;

pub static MIN_PASSWORD_LEN: Lazy<usize> = Lazy::new(|| {
    let min_password_len_str: &str = dotenv!("MIN_PASSWORD_LEN");
    min_password_len_str
        .parse::<usize>()
        .expect("The value provided in the .env is not a number!")
});

#[derive(Debug, PartialEq, Eq)]
pub enum Strength {
    Weak,
    Good,
    Strong,
}

/// Punctuation, lowercase chars, length greater than 15 => Strong
/// Punctuation, lowercase chars, length greater than MIN_PASSWORD_LEN (10) => GOOD
/// Punctuation, lowercase chars, length smaller than MIN_PASSWORD_LEN (10) => WEAK
/// length >= 15 => GOOD
/// length <= 10 => WEAK
pub fn measure_password_strength(password: &str) -> Strength {
    let (mut lowercase, mut special_char) = (false, false);
    for char in password.chars() {
        if char.is_lowercase() {
            lowercase = true;
        }
        if char.is_ascii_punctuation() {
            special_char = true;
        }
    }

    if lowercase && special_char {
        if password.len() >= 15 {
            Strength::Strong
        } else if password.len() >= *MIN_PASSWORD_LEN {
            Strength::Good
        } else {
            Strength::Weak
        }
    } else if password.len() >= 15 {
        Strength::Good
    } else {
        Strength::Weak
    }
}

#[cfg(test)]
mod tests {
    use super::{measure_password_strength, Strength};

    #[test]
    fn test_measure_pw_strength_weak() {
        assert_eq!(measure_password_strength("asdfvas"), Strength::Weak,);

        assert_eq!(measure_password_strength("Adf%vas"), Strength::Weak,)
    }

    #[test]
    fn test_measure_pw_strength_good() {
        assert_eq!(
            measure_password_strength("asdfasdfjkasfdjkasjfdklvas"),
            Strength::Good,
        );

        assert_eq!(measure_password_strength("daDsajc%hAada"), Strength::Good,)
    }

    #[test]
    fn test_measure_pw_strength_strong() {
        assert_eq!(
            measure_password_strength("asaASHJKFASa?ss"),
            Strength::Strong,
        );
    }
}
