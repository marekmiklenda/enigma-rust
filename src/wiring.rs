use lazy_static::lazy_static;

use crate::{EnigmaChar, EnigmaError, EnigmaResult};

/// Struct representing internal wiring schema of rotors
pub struct Wiring {
    /// Array of character codes that correspond to the letter of alphabet at the same index
    pub wiring: [u8; 26],
    /// Reverse wiring, for decoding
    pub reverse_wiring: [u8; 26],
    /// Optional turnover position
    pub notch_1: Option<u8>,
    /// Optional turnover position
    pub notch_2: Option<u8>,
}

impl Wiring {
    /// Returns a wiring created from a provided template
    /// 
    /// # Arguments
    /// 
    /// * `template` – Array of 26 characters of the alphabet where each letter corresponds to the letter of alphabet at the same index
    /// * `notch_1` – Optional turnover position
    /// * `notch_2` – Optional turnover position
    pub fn new(
        template: [char; 26],
        notch_1: Option<char>,
        notch_2: Option<char>,
    ) -> EnigmaResult<Self> {
        let notch_1 = notch_1
            .map(EnigmaChar::try_from)
            .transpose()?
            .map(|x| x.internal);
        let notch_2 = notch_2
            .map(EnigmaChar::try_from)
            .transpose()?
            .map(|x| x.internal);

        let mut wiring = [0u8; 26];
        let mut reverse_wiring = [0u8; 26];
        for i in 0..=25 {
            wiring[i] = EnigmaChar::try_from(template[i])?.internal;

            let ichar = char::from(EnigmaChar {
                internal: i as u8,
                uppercase: true,
            });

            let reverse_char = template
                .iter()
                .position(|x| x.to_ascii_uppercase() == ichar)
                .ok_or(EnigmaError::InvalidNumber(i as u8))? as u8;

            reverse_wiring[i] = reverse_char;
        }

        Ok(Self {
            wiring,
            reverse_wiring,
            notch_1,
            notch_2,
        })
    }
}

impl Clone for Wiring {
    fn clone(&self) -> Self {
        Self {
            wiring: self.wiring,
            reverse_wiring: self.reverse_wiring,
            notch_1: self.notch_1,
            notch_2: self.notch_2,
        }
    }
}

lazy_static! {
    static ref I: Wiring = Wiring::new(
        [
            'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X',
            'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J',
        ],
        Some('Q'),
        None
    )
    .unwrap();
    static ref II: Wiring = Wiring::new(
        [
            'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q',
            'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E',
        ],
        Some('E'),
        None
    )
    .unwrap();
    static ref III: Wiring = Wiring::new(
        [
            'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I',
            'W', 'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
        ],
        Some('V'),
        None
    )
    .unwrap();
    static ref IV: Wiring = Wiring::new(
        [
            'E', 'S', 'O', 'V', 'P', 'Z', 'J', 'A', 'Y', 'Q', 'U', 'I', 'R', 'H', 'X', 'L', 'N',
            'F', 'T', 'G', 'K', 'D', 'C', 'M', 'W', 'B',
        ],
        Some('J'),
        None
    )
    .unwrap();
    static ref V: Wiring = Wiring::new(
        [
            'V', 'Z', 'B', 'R', 'G', 'I', 'T', 'Y', 'U', 'P', 'S', 'D', 'N', 'H', 'L', 'X', 'A',
            'W', 'M', 'J', 'Q', 'O', 'F', 'E', 'C', 'K',
        ],
        Some('Z'),
        None
    )
    .unwrap();
    static ref VI: Wiring = Wiring::new(
        [
            'J', 'P', 'G', 'V', 'O', 'U', 'M', 'F', 'Y', 'Q', 'B', 'E', 'N', 'H', 'Z', 'R', 'D',
            'K', 'A', 'S', 'X', 'L', 'I', 'C', 'T', 'W',
        ],
        Some('Z'),
        Some('M')
    )
    .unwrap();
    static ref VII: Wiring = Wiring::new(
        [
            'N', 'Z', 'J', 'H', 'G', 'R', 'C', 'X', 'M', 'Y', 'S', 'W', 'B', 'O', 'U', 'F', 'A',
            'I', 'V', 'L', 'P', 'E', 'K', 'Q', 'D', 'T',
        ],
        Some('Z'),
        Some('M')
    )
    .unwrap();
    static ref VIII: Wiring = Wiring::new(
        [
            'F', 'K', 'Q', 'H', 'T', 'L', 'X', 'O', 'C', 'B', 'J', 'S', 'P', 'D', 'Z', 'R', 'A',
            'M', 'E', 'W', 'N', 'I', 'U', 'Y', 'G', 'V',
        ],
        Some('Z'),
        Some('M')
    )
    .unwrap();
    static ref UKW_A: Wiring = Wiring::new(
        [
            'E', 'J', 'M', 'Z', 'A', 'L', 'Y', 'X', 'V', 'B', 'W', 'F', 'C', 'R', 'Q', 'U', 'O',
            'N', 'T', 'S', 'P', 'I', 'K', 'H', 'G', 'D',
        ],
        None,
        None
    )
    .unwrap();
    static ref UKW_B: Wiring = Wiring::new(
        [
            'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E',
            'B', 'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T',
        ],
        None,
        None
    )
    .unwrap();
    static ref UKW_C: Wiring = Wiring::new(
        [
            'F', 'V', 'P', 'J', 'I', 'A', 'O', 'Y', 'E', 'D', 'R', 'Z', 'X', 'W', 'G', 'C', 'T',
            'K', 'U', 'Q', 'S', 'B', 'N', 'M', 'H', 'L',
        ],
        None,
        None
    )
    .unwrap();
}

/// Enum holding standard wirings for the Enigma M3 machine
#[allow(non_camel_case_types)]
pub enum StandardWiring {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    UKW_A,
    UKW_B,
    UKW_C,
}

impl From<StandardWiring> for Wiring {
    fn from(w: StandardWiring) -> Self {
        match w {
            StandardWiring::I => I.clone(),
            StandardWiring::II => II.clone(),
            StandardWiring::III => III.clone(),
            StandardWiring::IV => IV.clone(),
            StandardWiring::V => V.clone(),
            StandardWiring::VI => VI.clone(),
            StandardWiring::VII => VII.clone(),
            StandardWiring::VIII => VIII.clone(),
            StandardWiring::UKW_A => UKW_A.clone(),
            StandardWiring::UKW_B => UKW_B.clone(),
            StandardWiring::UKW_C => UKW_C.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wiring() {
        let wiring: Wiring = StandardWiring::I.into();
        let strw: String = wiring
            .wiring
            .iter()
            .map(|i| {
                char::from(EnigmaChar {
                    internal: *i,
                    uppercase: true,
                })
            })
            .collect();
        let strrw: String = wiring
        .reverse_wiring
        .iter()
        .map(|i| {
            char::from(EnigmaChar {
                internal: *i,
                uppercase: true,
            })
        })
        .collect();

        println!("Normal wiring:   {}\nReversed wiring: {}", strw, strrw);

        assert_eq!("EKMFLGDQVZNTOWYHXUSPAIBRCJ", strw);
        assert_eq!("UWYGADFPVZBECKMTHXSLRINQOJ", strrw);
    }
}
