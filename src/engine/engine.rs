use std::cell::RefCell;
use std::time::{Duration, Instant};

use crate::engine::node::Node;
use crate::game::board::{BitIter64, Bitboard};
use crate::game::game::GameState;

pub struct Engine {
    pub depth: u8,                    // Max depth to search
    pub node_counter: RefCell<usize>, // <- shared mutability, no threading
    pub last_search_time: RefCell<Duration>,
}

impl Engine {
    pub fn new(depth: u8) -> Self {
        Engine {
            depth,
            node_counter: RefCell::new(0),
            last_search_time: RefCell::new(Duration::ZERO),
        }
    }

    pub fn search(&self, state: &GameState) -> (i8, Option<u64>) {
        *self.node_counter.borrow_mut() = 0;
        let start_time = Instant::now();

        let is_white = state.current_player().to_bool();
        let node = Node::new(state.board, is_white, state.board.legal_moves(is_white));

        let mut best_move: Option<u64> = None;
        let mut best_score = if node.is_white { i8::MIN } else { i8::MAX };

        // use custom Bit iterator
        for mv in BitIter64(node.legal_moves) {
            let score = self.minimax(node.apply_move(mv), self.depth - 1, i8::MIN, i8::MAX);

            let is_better =
                (node.is_white && score > best_score) || (!node.is_white && score < best_score);

            if is_better {
                best_score = score;
                best_move = Some(mv);
            }
        }

        let duration = start_time.elapsed();
        *self.last_search_time.borrow_mut() = duration;

        (best_score, best_move)
    }

    fn minimax(&self, node: Node, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
        *self.node_counter.borrow_mut() += 1;

        // depth 0 or game over
        if (depth == 0) || (node.legal_moves == 0) {
            return static_eval(&node.board);
        }

        if node.is_white {
            let mut best_eval = i8::MIN;

            for mv in BitIter64(node.legal_moves) {
                let eval = self.minimax(node.apply_move(mv), depth - 1, alpha, beta);
                best_eval = best_eval.max(eval);
                alpha = alpha.max(best_eval);

                if beta <= alpha { return best_eval; }
            }
            return best_eval;
        } else {
            let mut best_eval = i8::MAX;

            for mv in BitIter64(node.legal_moves) {
                let eval = self.minimax(node.apply_move(mv), depth - 1, alpha, beta);
                best_eval = best_eval.min(eval);
                beta = beta.min(best_eval);

                if beta <= alpha { return best_eval; }
            }
            return best_eval;
        }
    }
}

#[inline(always)]
fn static_eval(board: &Bitboard) -> i8 {
    let (white, black) = board.score();
    return white - black;
}