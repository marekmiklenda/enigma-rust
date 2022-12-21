use enigma::{steckerbrett, wiring::StandardWiring, Enigma, EnigmaError};

#[test]
fn test_enigma_rotors() {
    let mut enigma = Enigma::standard(
        StandardWiring::UKW_B,
        StandardWiring::I,
        StandardWiring::II,
        StandardWiring::III,
        steckerbrett!(),
    );

    enigma.set_position_str("AAT").unwrap();
    enigma.get_for_char('A').unwrap();
    assert_eq!(enigma.get_position_str(), "AAU");
    enigma.get_for_char('A').unwrap();
    assert_eq!(enigma.get_position_str(), "AAV");
    enigma.get_for_char('A').unwrap();
    assert_eq!(enigma.get_position_str(), "ABW");

    enigma.set_position_str("AEU").unwrap();
    enigma.get_for_char('A').unwrap();
    assert_eq!(enigma.get_position_str(), "BFV");
}

#[test]
fn test_enigma_chars() {
    let mut enigma = Enigma::standard(
        StandardWiring::UKW_B,
        StandardWiring::I,
        StandardWiring::II,
        StandardWiring::III,
        steckerbrett!(),
    );

    enigma.set_position_str("AET").unwrap();

    let a = enigma.get_for_char('A').unwrap();
    let b = enigma.get_for_char('B').unwrap();
    let c = enigma.get_for_char('c').unwrap();

    assert_eq!(a, 'B');
    assert_eq!(b, 'A');
    assert_eq!(c, 'z');

    enigma.set_position_str("AET").unwrap();

    assert_eq!('A', enigma.get_for_char(a).unwrap());
    assert_eq!('B', enigma.get_for_char(b).unwrap());
    assert_eq!('c', enigma.get_for_char(c).unwrap());

    assert!(matches!(
        enigma.get_for_char('#'),
        Err(EnigmaError::UnsupportedCharacter('#'))
    ));

    assert_eq!(enigma.get_position_str(), "BGW");
}

#[test]
fn test_enigma_str() {
    let mut enigma = Enigma::standard(
        StandardWiring::UKW_B,
        StandardWiring::I,
        StandardWiring::II,
        StandardWiring::III,
        steckerbrett!(),
    );

    enigma.set_position_str("AET").unwrap();

    let enc1 = enigma.get_for_str("Bida Leonardovi", true, true).unwrap();
    let enc2 = enigma.get_for_str("Bida Leonardovi", false, false).unwrap();

    assert_eq!("Agqn Qyieuwbxsb", enc1);
    assert_eq!("ZEWFCIHPTEYSOF", enc2);

    enigma.set_position_str("AET").unwrap();

    assert_eq!(
        "Bida Leonardovi",
        enigma.get_for_str(&enc1, true, true).unwrap()
    );
    assert_eq!(
        "BIDALEONARDOVI",
        enigma.get_for_str(&enc2, true, true).unwrap()
    );

    assert_eq!(enigma.get_position_str(), "BGV");
}

#[test]
fn test_enigma_stecker() {
    let mut enigma_steck = Enigma::standard(
        StandardWiring::UKW_B,
        StandardWiring::I,
        StandardWiring::II,
        StandardWiring::III,
        steckerbrett!('X' => 'Q').unwrap(),
    );

    let mut enigma_nosteck = Enigma::standard(
        StandardWiring::UKW_B,
        StandardWiring::I,
        StandardWiring::II,
        StandardWiring::III,
        steckerbrett!(),
    );

    const TEST_STR: &str = "bida leonardovi";

    enigma_steck.set_position_str("AET").unwrap();
    enigma_nosteck.set_position_str("AET").unwrap();

    let enc1 = enigma_steck.get_for_str(TEST_STR, true, true).unwrap();
    let enc2 = enigma_nosteck.get_for_str(TEST_STR, true, true).unwrap();

    assert_eq!("agxn xyieuwbqsb", enc1);
    assert_eq!("agqn qyieuwbxsb", enc2);

    assert_ne!(enc1, enc2);

    enigma_steck.set_position_str("AET").unwrap();
    enigma_nosteck.set_position_str("AET").unwrap();

    assert_eq!(
        TEST_STR,
        enigma_steck.get_for_str(&enc1, true, true).unwrap()
    );
    assert_eq!(
        TEST_STR,
        enigma_nosteck.get_for_str(&enc2, true, true).unwrap()
    );
}
