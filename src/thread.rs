use crate::position::{CastleInfo, Position};
use crate::search::{start_search, Limits};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

//What we are doing is highly unsafe and
//probably doomed upon by any Rust community member

pub struct UnsafePtr<T>(*mut T);
unsafe impl<T> Send for UnsafePtr<T> {}

#[repr(C, align(64))]
#[derive(Clone, Copy, Default)]
pub struct Node(u64);

pub struct SharedState {
    node_counts: Arc<UnsafeCell<Vec<Node>>>,
    pub abort: Arc<AtomicBool>,
    txs: Vec<Sender<Instruction>>,
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
    fn reset_nodes(&self) {
        unsafe {
            for i in 0..self.txs.len() {
                self.node_counts.get().as_mut().unwrap()[i].0 = 0;
            }
        }
    }

    pub fn launch_threads(&mut self, threads: usize) {
        self.txs.iter().for_each(|x| {
            x.send(Instruction::Quit).unwrap();
        });
        unsafe { *self.node_counts.get().as_mut().unwrap() = vec![Node(0); threads] };
        self.txs = Vec::new();
        for _ in 0..threads {
            let (tx, rx) = channel();
            self.txs.push(tx);
            thread::spawn(move || worker_main(rx));
        }
    }

    pub fn start_search(&mut self, pos: Position, ci: CastleInfo, limits: Limits) {
        self.abort.store(false, Ordering::Relaxed);
        self.reset_nodes();
        for (id, sender) in self.txs.iter().enumerate() {
            unsafe {
                sender
                    .send(Instruction::Search(Thread {
                        node_counts: self.node_counts.clone(),
                        id,
                        nodes: UnsafePtr(
                            self.node_counts
                                .get()
                                .as_mut()
                                .unwrap()
                                .as_mut_ptr()
                                .add(id),
                        ),
                        ci: ci.clone(),
                        root: pos.clone(),
                        limits: limits.clone(),
                        abort: false,
                        global_abort: self.abort.clone(),
                    }))
                    .unwrap();
            }
        }
    }
}

fn worker_main(rx: Receiver<Instruction>) {
    while let Ok(instr) = rx.recv() {
        match instr {
            Instruction::Quit => {
                break;
            }
            Instruction::Search(thread) => {
                start_search(thread);
            }
        }
    }
}

pub enum Instruction {
    Quit,
    Search(Thread),
}
unsafe impl Send for Instruction {}

pub struct Thread {
    pub node_counts: Arc<UnsafeCell<Vec<Node>>>, //Only relevant for thread with id=0
    pub id: usize,
    pub nodes: UnsafePtr<Node>,
    pub root: Position,
    pub ci: CastleInfo,

    pub limits: Limits,
    pub abort: bool,
    pub global_abort: Arc<AtomicBool>,
}

impl Thread {
    pub fn inc_nodes(&self) {
        unsafe { (*self.nodes.0).0 += 1 };
    }

    pub fn bump_nodes(&self, bump: u64) {
        unsafe { (*self.nodes.0).0 += bump };
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
