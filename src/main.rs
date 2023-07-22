mod board;
mod piece;

use board::Board;
use piece::PieceType;

fn main() {
    // It works!
    let mut s = String::new();
    let b = Board::from_fen("rnbqkbnr/pp1pppp1/8/2p4p/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2
    ");

    for y in b.table {
        for x in y {
            s += &(x.map_or(' ', |a| match a.piece_type {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                PieceType::King => 'k',
            }).to_string());
        }
        s += "\n";
    }

    println!("{}", s);
}
