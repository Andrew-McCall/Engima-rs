impl super::ToEnigmaInt<char, usize> for char {
    /// This mimics turning a key press into an internal wire - which the rotors/plugboard will cipher
    fn to_internal_int(self) -> usize {
        // Ugly but efficient
        // Can use .to_ascii as usize - 65 to get the same result
        match self {
            'A' | 'a' => 0,
            'B' | 'b' => 1,
            'C' | 'c' => 2,
            'D' | 'd' => 3,
            'E' | 'e' => 4,
            'F' | 'f' => 5,
            'G' | 'g' => 6,
            'H' | 'h' => 7,
            'I' | 'i' => 8,
            'J' | 'j' => 9,
            'K' | 'k' => 10,
            'L' | 'l' => 11,
            'M' | 'm' => 12,
            'N' | 'n' => 13,
            'O' | 'o' => 14,
            'P' | 'p' => 15,
            'Q' | 'q' => 16,
            'R' | 'r' => 17,
            'S' | 's' => 18,
            'T' | 't' => 19,
            'U' | 'u' => 20,
            'V' | 'v' => 21,
            'W' | 'w' => 22,
            'X' | 'x' => 23,
            'Y' | 'y' => 24,
            'Z' | 'z' => 25,
            _ => panic!("Invalid character: {}", self),
        }
    }
}
