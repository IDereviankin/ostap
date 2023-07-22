use crate::piece::*;

#[derive(Debug)]
pub struct Board {
    pub table: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn from_fen(fen: impl Into<String>) -> Self {
        let mut result = Board { table: [[None; 8]; 8] }; 

        // I am not sure if cloning FEN string is right idea but it lets
        // the agrument type be flexible and not bound to a single type
        let owned_fen: String = fen.into();
        let fields: [&str; 6] = owned_fen
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let piece_placement = fields[0];

        let rows: [&str; 8] = piece_placement
            .trim()
            .split('/')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for (y, &row) in rows.iter().enumerate() {
            let mut offset: usize = 0;
            for (x, symbol) in row.chars().enumerate() {
                if symbol.is_digit(10) {
                    offset += symbol.to_digit(10).unwrap() as usize - 1;
                    continue;
                }
                result.table[y][x + offset] = Some(Piece::from(symbol));
            }
        }

        result
    }
}
