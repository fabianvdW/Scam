use crate::eval::eval;
use crate::position::*;

pub fn search(pos: &Position, ci: &CastleInfo, depth: i32) -> i32 {
    let mut best_score = -10000;

    if depth == 0 {
        return eval(pos);
    }

    for mv in pos.gen_pseudo_legals(ci) {
        let mut new_pos = pos.clone();
        if !new_pos.make_move(mv, ci) {
            continue;
        }

        let score = -search(&new_pos, ci, depth - 1);

        best_score = best_score.max(score);
    }

    best_score
}
