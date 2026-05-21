pub mod absences;
pub mod activity;
pub mod age;
pub mod call_to_orders;
pub mod complexity;
pub mod division_accuracy_score;
pub mod error;
pub mod filtering;
pub mod political_orientation;
pub mod session_activity;
pub mod speeches;

pub(crate) fn legislative_period_rank(period: Option<&str>) -> i32 {
    let Some(period) = period else {
        return i32::MIN;
    };

    if let Ok(rank) = period.parse::<i32>() {
        return rank;
    }

    roman_to_int(period).unwrap_or(i32::MIN)
}

fn roman_to_int(value: &str) -> Option<i32> {
    let mut total = 0;
    let mut previous = 0;

    for char in value.chars().rev() {
        let current = match char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return None,
        };

        if current < previous {
            total -= current;
        } else {
            total += current;
        }
        previous = current;
    }

    Some(total)
}

pub use absences::*;
pub use activity::*;
pub use age::*;
pub use call_to_orders::*;
pub use complexity::*;
pub use division_accuracy_score::*;
pub use error::*;
pub use filtering::*;
pub use political_orientation::*;
pub use session_activity::*;
pub use speeches::*;
