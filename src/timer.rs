use std::{sync::{Arc, Mutex, mpsc::{Receiver, Sender}}, thread, time::{Duration, Instant}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimerStatus {
    Running,
    Paused,
    Stopped,
    Finished
}

#[derive(Clone)]
pub struct Timer {
    pub current_time: u8,
    pub status: TimerStatus,
    pub paused_since: u64
}

impl Timer {

    pub fn new(duration: u8) -> Self {
        Self {
            current_time: duration,
            status: TimerStatus::Running,
            paused_since: 0
        }
    }

    pub fn set_status(&mut self, status: TimerStatus) {
        self.status = status;
    }

    pub fn start(&self, tx: Sender<u64>, status_rx: Receiver<TimerStatus>) {
        let mut me = self.clone();

        tx.send(me.current_time as u64).unwrap();

        // Boucle pour le timer
        thread::spawn(move || {
            while me.current_time > 0 {
                std::thread::sleep(Duration::from_secs(1));
                if let Ok(recv) = status_rx.try_recv() {
                    me.set_status(recv);
                }

                match me.status {
                    TimerStatus::Stopped => {
                        break;
                    },
                    TimerStatus::Paused => {
                        me.paused_since += 1;
                        tx.send(me.paused_since).unwrap();
                    },
                    TimerStatus::Running => {
                        me.current_time -= 1;
                        tx.send(me.current_time as u64).unwrap();

                        if me.paused_since > 0 {
                            me.paused_since = 0;
                        }
                    },
                    _ => ()
                };
            };

            if let Ok(recv) = status_rx.recv() {
                me.set_status(recv);
            }

        });
    }
}