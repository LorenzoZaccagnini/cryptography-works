//taken inspiration from https://github.com/arosspope/cipher-crypt, almost a clone :)

pub mod helpers {
    use crate::alphabet::ALPHABET_LOWER;

    fn shift_substitution<F>(text: &str, calc_index: F) -> String
    where
        F: Fn(usize) -> usize,
    {
        let mut s_text = String::new();
        for c in text.chars() {
            //Find the index of the character in the alphabet (if it exists in there)
            let pos = ALPHABET_LOWER.iter().position(|&a| a == c);
            match pos {
                Some(pos) => {
                    let index = calc_index(pos); //Calculate substitution index
                    s_text.push(ALPHABET_LOWER[index]);
                }
                None => s_text.push(c), //Push non-alphabetic chars 'as-is'
            }
        }

        s_text
    }

    fn modulo(i: isize) -> usize {
        (((i % 26 as isize) + 26 as isize) % 26 as isize) as usize
    }

    pub fn encrypt(message: &str, shift: usize) -> Result<String, &'static str> {
        Ok(shift_substitution(message, |idx| {
            modulo((idx + shift) as isize)
        }))
    }
}
