/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBOpenDBRequestBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::reflect_dom_object;
use dom::idbrequest::IDBRequest;

#[dom_struct]
pub struct IDBOpenDBRequest {
    idbrequest: IDBRequest,
}

impl IDBOpenDBRequest {
    fn new_inherited() -> IDBOpenDBRequest {
        IDBOpenDBRequest {
            idbrequest: IDBRequest::new_inherited(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBOpenDBRequest> {
        reflect_dom_object(box IDBOpenDBRequest::new_inherited(),
                           global,
                           IDBOpenDBRequestBinding::Wrap)
    }
}
