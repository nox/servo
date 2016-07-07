/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use net_traits::indexeddb_thread::IndexedDbThreadMsg;
use std::collections::HashMap;
use util::thread::spawn_named;

pub trait IndexedDbThreadFactory {
    fn new() -> Self;
}

impl IndexedDbThreadFactory for IpcSender<IndexedDbThreadMsg> {
    /// Create an IndexedDB thread.
    fn new() -> IpcSender<IndexedDbThreadMsg> {
        let (sender, receiver) = ipc::channel().unwrap();
        spawn_named("IndexedDbManager".to_owned(), move || {
            IndexedDbManager::new(receiver).start();
        });
        sender
    }
}

struct IndexedDbManager {
    receiver: IpcReceiver<IndexedDbThreadMsg>,
    databases: HashMap<String, ()>,
}

impl IndexedDbManager {
    fn new(receiver: IpcReceiver<IndexedDbThreadMsg>) -> IndexedDbManager {
        IndexedDbManager {
            receiver: receiver,
            databases: HashMap::new(),
        }
    }
}

impl IndexedDbManager {
    fn start(&mut self) {
        loop {
            match self.receiver.recv().unwrap() {
                IndexedDbThreadMsg::Exit(sender) => {
                    let _ = sender.send(());
                    break
                }
            }
        }
    }
}
