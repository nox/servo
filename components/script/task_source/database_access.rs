/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use script_thread::{MainThreadScriptMsg, Runnable, ScriptThread};
use std::result::Result;
use std::sync::mpsc::Sender;
use task_source::TaskSource;

#[derive(JSTraceable, Clone)]
pub struct DatabaseAccessTaskSource(pub Sender<MainThreadScriptMsg>);

impl TaskSource<DatabaseAccessTask> for DatabaseAccessTaskSource {
    fn queue(&self, msg: DatabaseAccessTask) -> Result<(), ()> {
        self.0.send(MainThreadScriptMsg::DatabaseAccess(msg)).map_err(|_| ())
    }
}

pub enum DatabaseAccessTask {
    Runnable(Box<Runnable + Send>),
}

impl DatabaseAccessTask {
    pub fn handle_task(self, script_thread: &ScriptThread) {
        match self {
            DatabaseAccessTask::Runnable(runnable) => {
                if !runnable.is_cancelled() {
                    runnable.main_thread_handler(script_thread);
                }
            }
        }
    }
}
