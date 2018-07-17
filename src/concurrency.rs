use crossbeam_channel::{Receiver};

#[must_use]
pub struct ConcurrentJob {
    chan: Receiver<()>,
}

pub fn wait_for_all(mut jobs: Vec<ConcurrentJob>,) {
    while !jobs.is_empty() {
        let done: usize = {
            let chans = jobs.iter().map(|j| &j.chan);
            select!{
                recv(chans, msg, from) => {
                    assert!(msg.is_none());
                    jobs.iter().position(| j | & j.chan == from).unwrap()
                }
            }
        };
        drop(jobs.swap_remove(done));
    }
}
