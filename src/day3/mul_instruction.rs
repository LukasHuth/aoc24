use std::ops::AddAssign;

use crate::utilities::datatypes::num_wrapper::NumWrapper;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) struct MulInstruction {
    value_1: u32,
    value_2: u32,
}

impl MulInstruction {
    /// Converts a string in the format `mul(num_a, num_b)` where `num_a` and `num_b` are integers with 1-3 digits into an instance of `Self`.
    ///
    /// # Parameters
    /// - `data`: A string slice containing the input in the format `mul(num_a, num_b)`.
    ///
    /// # Returns
    /// - `Ok((Self, usize))`: On successful parsing, returns an instance of `Self` along with the number of characters used for parsing.
    /// - `Err(&'static str)`: Returns an error message if the input string does not match the expected format or if parsing fails.
    ///
    /// # Errors
    /// - If the string does not start with "mul(".
    /// - If the values `num_a` or `num_b` are not valid integers with 1-3 digits.
    /// - If the string does not contain a comma ',' separating `num_a` and `num_b`.
    /// - If the string does not end with a closing parenthesis ')'.
    ///
    /// # Example
    /// ```
    /// let input = "mul(123,456)";
    /// match MulInstruction::new(input) {
    ///     Ok((instruction, chars_used)) => {
    ///         println!("Parsed: {:?}, characters used: {}", instruction, chars_used);
    ///     }
    ///     Err(err) => {
    ///         println!("Error: {}", err);
    ///     }
    /// }
    /// ```
    pub fn new(data: &str) -> Result<(Self, usize), &'static str> {
        assert!(data.len() >= 8 && data.len() <= 12);
        if &data[0..4] != "mul(" {
            return Err("The string does not start with a mul(");
        }
        let mut value_1 = 0;
        let mut v_1_counter = 0;
        let v_1_offset = 4;
        while v_1_counter < 3 {
            let next_char = data
                .chars()
                .nth(v_1_offset + v_1_counter)
                .ok_or_else(|| "Could not get a char from the string for value 1")?;
            if !next_char.is_ascii_digit() {
                break;
            }
            value_1 *= 10;
            value_1 += next_char
                .to_digit(10)
                .ok_or_else(|| "The char to digit conversion failed")?;
            v_1_counter += 1;
        }
        if v_1_counter == 0 {
            return Err("v_1_counter is 0 so no value was read");
        }
        if data.chars().nth(v_1_offset + v_1_counter) != Some(',') {
            return Err("Expected an , after value_1");
        }
        let mut value_2 = 0;
        let mut v_2_counter = 0;
        let v_2_offset = v_1_offset + v_1_counter + 1;
        while v_2_counter < 3 {
            let next_char = data
                .chars()
                .nth(v_2_offset + v_2_counter)
                .ok_or_else(|| "Could not get a char from the string for value 1")?;
            if !next_char.is_ascii_digit() {
                break;
            }
            value_2 *= 10;
            value_2 += next_char
                .to_digit(10)
                .ok_or_else(|| "The char to digit conversion failed on value 2")?;
            v_2_counter += 1;
        }
        if v_2_counter == 0 {
            return Err("v_2_counter is 0 so no value was read");
        }
        if data.chars().nth(v_2_offset + v_2_counter) != Some(')') {
            return Err("The chain does not end with a )");
        }
        Ok((Self { value_1, value_2 }, v_2_offset + v_2_counter))
    }
}
impl<T> AddAssign<MulInstruction> for NumWrapper<T>
where
    NumWrapper<T>: AddAssign<T>,
    T: std::num::ZeroablePrimitive + From<u32>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: MulInstruction) {
        *self += T::from(rhs.value_1 * rhs.value_2);
    }
}
#[test]
fn test_mul_inst_new() -> Result<(), &'static str> {
    let start = std::time::Instant::now();
    let (test, _) = MulInstruction::new("mul(1,2)")?;
    let expected = MulInstruction {
        value_1: 1,
        value_2: 2,
    };
    assert_eq!(test, expected);
    println!("{:?}", start.elapsed());
    Ok(())
}
