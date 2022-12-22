use std::collections::HashMap;

use rotor::Rotor;
use wiring::{StandardWiring, Wiring};

mod rotor;
pub mod wiring;

/// Result returned by this crate's functions
pub type EnigmaResult<T> = Result<T, EnigmaError>;

/// Errors returned by this crate
#[derive(Debug)]
pub enum EnigmaError {
    InvalidChar(char),
    InvalidNumber(u8),
    InvalidPosition(String),
    InvalidSteckerbrettString(String),
    UnsupportedCharacter(char),
}

impl std::fmt::Display for EnigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "Cannot index using invalid character: {}", c),
            Self::InvalidNumber(c) => write!(f, "Cannot index using invalid number: {}", c),
            Self::InvalidPosition(s) => {
                write!(f, "String '{}' cannot be used to set position", s)
            }
            Self::UnsupportedCharacter(c) => write!(f, "Character '{}' cannot be encoded", c),
            Self::InvalidSteckerbrettString(s) => {
                write!(f, "String '{}' is not representing valid stecker pairs!", s)
            }
        }
    }
}

/// Struct representing a character while inside of the enigma machine
pub struct EnigmaChar {
    /// Position in the alphabet of this character
    pub internal: u8,
    /// Should this character be uppercase
    pub uppercase: bool,
}

impl TryFrom<char> for EnigmaChar {
    type Error = EnigmaError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let uppercase = match value as u8 {
            65..=90 => true,
            97..=122 => false,
            _ => return Err(EnigmaError::InvalidChar(value)),
        };

        Ok(Self {
            internal: value as u8 - if uppercase { 65 } else { 97 },
            uppercase,
        })
    }
}

impl TryFrom<&char> for EnigmaChar {
    type Error = EnigmaError;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

impl From<&EnigmaChar> for char {
    fn from(c: &EnigmaChar) -> Self {
        (c.internal + if c.uppercase { 65 } else { 97 }) as char
    }
}

impl From<EnigmaChar> for char {
    fn from(c: EnigmaChar) -> Self {
        Self::from(&c)
    }
}

impl PartialEq<char> for EnigmaChar {
    fn eq(&self, other: &char) -> bool {
        other == &char::from(self)
    }
}

impl PartialEq for EnigmaChar {
    fn eq(&self, other: &Self) -> bool {
        self.internal == other.internal && self.uppercase == other.uppercase
    }
}

/// Struct representing a plugboard
///
/// Use the steckerbrett! macro for construction unless advanced behaviour is needed.
pub struct Steckerbrett(pub HashMap<u8, u8>);

/// Macro for creating plugboards
///
/// # Examples
///
/// ```
/// use enigma::steckerbrett;
///
/// // Creates an empty plugboard
/// let s = steckerbrett!();
/// // Creates a plugboard with 'F' and 'R' connected
/// let s = steckerbrett!('F' => 'R');
/// // Creates the same plugboard from a slice
/// let s = steckerbrett!([('F', 'R')].as_slice());
/// // Creates the same plugboard from a string
/// let s = steckerbrett!("FR");
/// ```
#[macro_export]
macro_rules! steckerbrett {
    () => {
        $crate::Steckerbrett(std::collections::HashMap::new())
    };

    ($a:expr) => {
        $crate::Steckerbrett::try_from($a)
    };

    ($($a:expr => $b:expr),+) => {
        steckerbrett!([$(($a, $b)),+].as_slice())
    };
}

impl Steckerbrett {
    /// Run a character through this plugboard
    ///
    /// # Arguments
    /// * `char` - Input character
    pub fn get(&self, char: &mut EnigmaChar) {
        if let Some(val) = self.0.get(&char.internal) {
            char.internal = *val;
        }
    }
}

impl TryFrom<&[(char, char)]> for Steckerbrett {
    type Error = EnigmaError;
    fn try_from(value: &[(char, char)]) -> Result<Self, Self::Error> {
        let mut z = steckerbrett!();

        for (c, d) in value.iter() {
            let c = EnigmaChar::try_from(c)?;
            let d = EnigmaChar::try_from(d)?;

            z.0.insert(c.internal, d.internal);
            z.0.insert(d.internal, c.internal);
        }

        Ok(z)
    }
}

impl TryFrom<&Vec<(char, char)>> for Steckerbrett {
    type Error = EnigmaError;
    fn try_from(value: &Vec<(char, char)>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

impl TryFrom<&str> for Steckerbrett {
    type Error = EnigmaError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(
            &value
                .split_whitespace()
                .map(|p| {
                    if p.len() != 2 {
                        None
                    } else {
                        let mut c = p.chars();
                        Some((c.next().unwrap(), c.next().unwrap()))
                    }
                })
                .collect::<Option<Vec<(char, char)>>>()
                .ok_or_else(|| EnigmaError::InvalidPosition(value.to_owned()))?,
        )
    }
}

/// Struct representing a fully defined M3 Enigma machine.
///
/// # Examples
///
/// ```
/// use enigma::{Enigma, steckerbrett, wiring::StandardWiring};
///
/// // Creates an enigma
/// let mut enigma = Enigma::standard(
///     StandardWiring::UKW_B,
///     StandardWiring::I,
///     StandardWiring::II,
///     StandardWiring::III,
///     steckerbrett!('A' => 'Q', 'F' => 'R', 'S' => 'M').unwrap(),
/// );
///
/// /// Encode "test"
/// let enc = enigma.get_for_str("test", false, true).unwrap();
///
/// assert_eq!("olkr", enc);
/// ```
pub struct Enigma {
    /// Reflector rotor
    ukw: Rotor,
    /// Left rotor (rotor 1)
    rotor_l: Rotor,
    /// Middle rotor (rotor 2)
    rotor_m: Rotor,
    /// Right rotor (rotor 3)
    rotor_r: Rotor,
    /// Plugboard
    steckerbrett: Steckerbrett,
}

impl Enigma {
    /// Creates a new enigma machine with the specified custom wirings.
    ///
    /// If you don't need to specify a custom wiring, using Enigma::standard() is preferred.
    ///
    /// # Arguments
    ///
    /// * `ukw` - Wiring of the reflector
    /// * `wiring_l` - Wiring of the left rotor (rotor 1)
    /// * `wiring_m` - Wiring of the middle rotor (rotor 2)
    /// * `wiring_r` - Wiring of the right rotor (rotor 3)
    /// * `stecker` - Plugboard
    pub fn new(
        ukw: Wiring,
        wiring_l: Wiring,
        wiring_m: Wiring,
        wiring_r: Wiring,
        stecker: Steckerbrett,
    ) -> Self {
        Self {
            ukw: Rotor::new(ukw),
            rotor_l: Rotor::new(wiring_l),
            rotor_m: Rotor::new(wiring_m),
            rotor_r: Rotor::new(wiring_r),
            steckerbrett: stecker,
        }
    }

    /// Creates a new Enigma M3 machine with the specified standard wirings
    ///
    /// # Arguments
    ///
    /// * `ukw` - Wiring of the reflector
    /// * `wiring_l` - Wiring of the left rotor (rotor 1)
    /// * `wiring_m` - Wiring of the middle rotor (rotor 2)
    /// * `wiring_r` - Wiring of the right rotor (rotor 3)
    /// * `stecker` - Plugboard
    ///
    /// /// # Examples
    ///
    /// ```
    /// use enigma::{Enigma, steckerbrett, wiring::StandardWiring};
    ///
    /// // Creates an enigma machine
    /// let mut enigma = Enigma::standard(
    ///     StandardWiring::UKW_B,
    ///     StandardWiring::I,
    ///     StandardWiring::II,
    ///     StandardWiring::III,
    ///     steckerbrett!('A' => 'Q', 'F' => 'R', 'S' => 'M').unwrap(),
    /// );
    /// ```
    pub fn standard(
        ukw: StandardWiring,
        wiring_l: StandardWiring,
        wiring_m: StandardWiring,
        wiring_r: StandardWiring,
        stecker: Steckerbrett,
    ) -> Self {
        Self::new(
            ukw.into(),
            wiring_l.into(),
            wiring_m.into(),
            wiring_r.into(),
            stecker,
        )
    }

    /// Sets the rotor's positions
    ///
    /// # Arguments
    ///
    /// * `rotor_l` - Position of the left rotor (rotor 1)
    /// * `rotor_m` - Position of the middle rotor (rotor 2)
    /// * `rotor_r` - Position of the right rotor (rotor 3)
    pub fn set_position(
        &mut self,
        rotor_l: Option<char>,
        rotor_m: Option<char>,
        rotor_r: Option<char>,
    ) -> EnigmaResult<()> {
        if let Some(c) = rotor_l {
            self.rotor_l.set_position(&EnigmaChar::try_from(c)?)?;
        }

        if let Some(c) = rotor_m {
            self.rotor_m.set_position(&EnigmaChar::try_from(c)?)?;
        }

        if let Some(c) = rotor_r {
            self.rotor_r.set_position(&EnigmaChar::try_from(c)?)?;
        }

        Ok(())
    }

    /// Sets the rotor's positions specified by a string.
    ///
    /// # Arguments
    ///
    /// * `position` - A three long string of ascii alphabet characters, each representing a rotor's position. Left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// use enigma::{Enigma, steckerbrett, wiring::StandardWiring};
    ///
    /// // Creates an enigma machine
    /// let mut enigma = Enigma::standard(
    ///     StandardWiring::UKW_B,
    ///     StandardWiring::I,
    ///     StandardWiring::II,
    ///     StandardWiring::III,
    ///     steckerbrett!('A' => 'Q', 'F' => 'R', 'S' => 'M').unwrap(),
    /// );
    ///
    /// // Sets the left rotor to 'F', middle to 'C' and right to 'B'
    /// enigma.set_position_str("FCB");
    ///
    /// // Encode "test" starting at "FCB"
    /// let pos_fcb = enigma.get_for_str("test", false, true).unwrap();
    ///
    /// // Sets all rotors to 'A'
    /// enigma.set_position_str("AAA");
    ///
    /// // Encode "test" starting at "AAA"
    /// let pos_aaa = enigma.get_for_str("test", false, true).unwrap();
    ///
    /// assert_ne!(pos_fcb, pos_aaa);
    /// ```
    pub fn set_position_str(&mut self, position: &str) -> EnigmaResult<()> {
        if position.len() != 3 {
            return Err(crate::EnigmaError::InvalidPosition(position.to_owned()));
        }

        let mut chars = position.chars();

        self.set_position(chars.next(), chars.next(), chars.next())
    }

    /// Returns the positions of the rotors as a three-long array. Index 0 is the left rotor, index 1 is the middle rotor and index 2 is the right rotor.
    pub fn get_position(&self) -> [char; 3] {
        [
            char::from(self.rotor_l.get_position()),
            char::from(self.rotor_m.get_position()),
            char::from(self.rotor_r.get_position()),
        ]
    }

    /// Returns the position of the rotors as a three-long string. First character is the left rotor, second is the middle rotor and the third is the right rotor.
    pub fn get_position_str(&self) -> String {
        let pos = self.get_position();
        format!("{}{}{}", pos[0], pos[1], pos[2])
    }

    /// Rotates the rotors by one step
    fn turn_rotors(&mut self) {
        let notch_r = self.rotor_r.has_notch();
        let notch_m = self.rotor_m.has_notch();

        self.rotor_r.rotate();
        if notch_r {
            self.rotor_m.rotate();
        }
        if notch_m {
            self.rotor_m.rotate();
            self.rotor_l.rotate();
        }
    }

    /// Runs a single character through the machine
    ///
    /// # Arguments
    ///
    /// * `c` - Character to encode
    pub fn get_for_char(&mut self, c: char) -> EnigmaResult<char> {
        self._internal_get_for_char(c).map(char::from)
    }

    /// Actually runs a single character through a machine, only difference being that this method returns an ```EnigmaCharacter```.
    ///
    /// # Arguments
    ///
    /// * `c` - Character to encode
    fn _internal_get_for_char(&mut self, c: char) -> EnigmaResult<EnigmaChar> {
        let mut c = {
            let x = EnigmaChar::try_from(c);
            if let Err(EnigmaError::InvalidChar(c)) = x {
                return Err(EnigmaError::UnsupportedCharacter(c));
            }

            x?
        };

        self.turn_rotors();

        self.steckerbrett.get(&mut c);
        self.rotor_r.get_for(&mut c, false)?;
        self.rotor_m.get_for(&mut c, false)?;
        self.rotor_l.get_for(&mut c, false)?;

        self.ukw.get_for(&mut c, false)?;

        self.rotor_l.get_for(&mut c, true)?;
        self.rotor_m.get_for(&mut c, true)?;
        self.rotor_r.get_for(&mut c, true)?;
        self.steckerbrett.get(&mut c);

        Ok(c)
    }

    /// Encodes a string using this enigma machine.
    ///
    /// # Arguments
    ///
    /// * `str` - String to encrypt
    /// * `preserve_unsupported` - Whether non-alphabet characters should be preserved in the output
    /// * `preserve_case` - Whether output characters should match the case of the input characters
    ///
    /// # Examples
    ///
    /// ```
    /// use enigma::{Enigma, steckerbrett, wiring::StandardWiring};
    ///
    /// // Creates an enigma machine
    /// let mut enigma = Enigma::standard(
    ///     StandardWiring::UKW_B,
    ///     StandardWiring::I,
    ///     StandardWiring::II,
    ///     StandardWiring::III,
    ///     steckerbrett!('A' => 'Q', 'F' => 'R', 'S' => 'M').unwrap(),
    /// );
    ///
    /// /// Encode "test"
    /// let enc = enigma.get_for_str("test", false, true).unwrap();
    ///
    /// assert_eq!("olkr", enc);
    /// ```
    pub fn get_for_str(
        &mut self,
        str: &str,
        preserve_unsupported: bool,
        preserve_case: bool,
    ) -> EnigmaResult<String> {
        let mut out = String::new();

        for c in str.chars() {
            match self._internal_get_for_char(c) {
                Ok(mut c) => {
                    if !preserve_case {
                        c.uppercase = true;
                    }

                    out.push(char::from(c))
                }
                Err(crate::EnigmaError::UnsupportedCharacter(c)) => {
                    if preserve_unsupported {
                        out.push(c);
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stecker() {
        assert!(
            steckerbrett!("AE IO ML").unwrap().0
                == steckerbrett!('A' => 'E', 'I' => 'O', 'M' => 'L').unwrap().0,
        );
    }
}
