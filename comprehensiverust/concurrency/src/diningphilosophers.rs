// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: Philosopher
use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    // ANCHOR_END: Philosopher
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
    id: usize,
}

// ANCHOR: Philosopher-think
impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }
    // ANCHOR_END: Philosopher-think

    // ANCHOR: Philosopher-eat
    fn eat(&self) {
        // ANCHOR_END: Philosopher-eat
        println!("{} is trying to eat", &self.name);
        if self.id % 2 != 0 {
            let _left = self.left_fork.lock().unwrap();
            let _right = self.right_fork.lock().unwrap();
        } else {
            let _right = self.right_fork.lock().unwrap();
            let _left = self.left_fork.lock().unwrap();
        }

        // ANCHOR: Philosopher-eat-end
        println!("{} is eating...", &self.name);
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(0..10)));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

pub fn main() {
    // ANCHOR_END: Philosopher-eat-end
    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    for i in 0..forks.len() {
        let tx = tx.clone();
        let left_fork = forks[i].clone();
        let right_fork = forks[(i + 1) % forks.len()].clone();

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
            id: i,
        };

        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.think();
                philosopher.eat();
            }
        });
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }
}
