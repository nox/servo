/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ipc_channel::ipc::IpcSender;

/// Messages sent to the IndexedDB thread.
#[derive(Deserialize, Serialize)]
pub enum IndexedDbThreadMsg {
    /// Exit the IndexedDB thread.
    Exit(IpcSender<()>)
}
