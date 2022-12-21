use crate::EnigmaChar;

use super::wiring::Wiring;
use super::EnigmaResult;

/// Struct representing a rotor inside an enigma machine
pub struct Rotor {
    /// Internal wiring of the rotor
    wiring: Wiring,
    /// Current position of this rotor
    position: u8,
}

impl Rotor {
    /// Creates a new rotor
    /// 
    /// # Arguments
    /// *wiring* - Internal wiring of the rotor
    pub fn new(wiring: Wiring) -> Self {
        Self {
            wiring,
            position: 0,
        }
    }

    /// Rotates the rotor once
    pub fn rotate(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    /// Sets the rotor's position
    /// 
    /// # Arguments
    /// *pos* - Target position
    pub fn set_position(&mut self, pos: &EnigmaChar) -> EnigmaResult<()> {
        self.position = pos.internal;

        Ok(())
    }

    /// Returns true if the rotor is currently on it's turnover notch
    pub fn has_notch(&self) -> bool {
        matches!(self.wiring.notch_1, Some(x) if x == self.position)
            || matches!(self.wiring.notch_2, Some(x) if x == self.position)
    }

    /// Returns the rotor's current position
    pub fn get_position(&self) -> EnigmaChar {
        EnigmaChar {
            internal: self.position,
            uppercase: true,
        }
    }

    /// Runs an input through this rotor
    /// 
    /// # Arguments
    /// *input* - Character to encode
    /// *reversed* - Whether to use the reverse wiring for signals travelling backwards
    pub fn get_for(&self, input: &mut EnigmaChar, reversed: bool) -> EnigmaResult<()> {
        let inchar = (input.internal + self.position) % 26;

        let outchar = (if reversed {
            &self.wiring.reverse_wiring
        } else {
            &self.wiring.wiring
        }[inchar as usize]
            + 26
            - self.position)
            % 26;

        input.internal = outchar;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::wiring::StandardWiring;

    use super::*;

    macro_rules! assert_rotor {
        ($r:expr, $a:literal, $b:literal, $c:literal) => {{
            if let Ok(mut a) = EnigmaChar::try_from($a) {
                $r.get_for(&mut a, $c).unwrap();
                assert_eq!(char::from(a), $b)
            }else {
                assert_eq!($a, $b);
            }
        }};
    }

    #[test]
    fn test_wiring() {
        let mut rotor1 = Rotor::new(StandardWiring::I.into());

        rotor1
            .set_position(&EnigmaChar::try_from('A').unwrap())
            .unwrap();
        assert_rotor!(rotor1, 'A', 'E', false);
        assert_rotor!(rotor1, 'z', 'j', false);
        assert_rotor!(rotor1, 'E', 'A', true);
        assert_rotor!(rotor1, 'j', 'z', true);
        
        assert_rotor!(rotor1, '#', '#', false);

        rotor1
            .set_position(&EnigmaChar::try_from('B').unwrap())
            .unwrap();
        assert_rotor!(rotor1, 'A', 'J', false);
        assert_rotor!(rotor1, 'z', 'd', false);
        assert_rotor!(rotor1, 'J', 'A', true);
        assert_rotor!(rotor1, 'd', 'z', true);
    }
}
