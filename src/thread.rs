use crate::history::HashHist;
use crate::position::{CastleInfo, Position};
use crate::r#move::*;
use crate::search::{start_search, Limits};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub struct UnsafePtr<T>(*mut T);
unsafe impl<T> Send for UnsafePtr<T> {}

#[repr(C, align(64))]
#[derive(Clone, Copy, Default)]
pub struct Node(u64);

/*
Memory model of SharedState and Threads
SharedState:

  node_counts = [ . , . , . , .]        abort: bool                txs = [., ., . , . , .]
          ^       ^   ^                    ^                              ^  ^
          | <     |   |                    | <                            |  |
Thread 0: . |     .   |                    . |                     rx   <--  |
Thread 1:   .         .                      .                     rx <------
Field: node_counts  nodes                  abort                   rx
 */
pub struct SharedState {
    node_counts: Arc<UnsafeCell<Vec<Node>>>,
    pub abort: Arc<AtomicBool>,
    txs: Vec<Sender<Option<Thread>>>,
}

impl Default for SharedState {
    fn default() -> Self {
        SharedState {
            node_counts: Arc::new(UnsafeCell::new(Vec::new())),
            abort: Arc::new(AtomicBool::new(false)),
            txs: Vec::new(),
        }
    }
}
impl SharedState {
    pub fn reset_nodes(&self) {
        unsafe {
            for i in 0..self.txs.len() {
                self.node_counts.get().as_mut().unwrap()[i].0 = 0;
            }
        }
    }

    pub fn launch_threads(&mut self, threads: usize) {
        self.txs.iter().for_each(|x| {
            x.send(None).unwrap();
        });
        unsafe { *self.node_counts.get().as_mut().unwrap() = vec![Node(0); threads] };
        self.txs = Vec::new();
        for _ in 0..threads {
            let (tx, rx) = channel();
            self.txs.push(tx);
            thread::spawn(move || worker_main(rx));
        }
    }

    pub fn start_search(&mut self, pos: Position, ci: CastleInfo, hist: HashHist, limits: Limits) {
        self.abort.store(false, Ordering::Relaxed);
        self.reset_nodes();
        for (id, sender) in self.txs.iter().enumerate() {
            let (pos, ci, hist, limits) = (pos.clone(), ci.clone(), hist.clone(), limits.clone());
            sender
                .send(Some(Thread::new(self, id, pos, ci, hist, limits)))
                .unwrap();
        }
    }
}

fn worker_main(rx: Receiver<Option<Thread>>) {
    while let Ok(Some(mut t)) = rx.recv() {
        start_search(&mut t);
    }
}

pub struct Thread {
    pub node_counts: Arc<UnsafeCell<Vec<Node>>>, //Only relevant for thread with id=0
    pub id: usize,
    pub nodes: UnsafePtr<Node>,
    pub abort: Arc<AtomicBool>,
    pub limits: Limits,

    pub root: Position,
    pub ci: CastleInfo,
    pub hist: HashHist,
    pub best_move: Move,
}
unsafe impl Send for Thread {}

impl Thread {
    #[rustfmt::skip]
    pub fn new(
        shared_state: &SharedState, id: usize,
        root: Position, ci: CastleInfo, hist: HashHist, limits: Limits,
    ) -> Self {
        unsafe {
            let ptr = shared_state.node_counts.get().as_mut().unwrap();
            let nodes = UnsafePtr(ptr.as_mut_ptr().add(id));
            let (node_counts, abort) =
                (shared_state.node_counts.clone(), shared_state.abort.clone());
            let best_move = NO_MOVE;

            Thread {
                id, nodes, node_counts, root, ci,
                best_move,limits, abort, hist
            }
        }
    }

    pub fn inc_nodes(&self) {
        unsafe { (*self.nodes.0).0 += 1 };
    }

    pub fn get_local_nodes(&self) -> u64 {
        unsafe { (*self.nodes.0).0 }
    }

    pub fn get_global_nodes(&self) -> u64 {
        unsafe {
            self.node_counts
                .get()
                .as_ref()
                .unwrap()
                .iter()
                .fold(0, |acc, x| acc + x.0)
        }
    }
}
