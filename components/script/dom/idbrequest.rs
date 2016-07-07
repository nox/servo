/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBRequestBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::reflect_dom_object;
use dom::eventtarget::EventTarget;

#[dom_struct]
pub struct IDBRequest {
    eventtarget: EventTarget,
}

impl IDBRequest {
    fn new_inherited() -> IDBRequest {
        IDBRequest {
            eventtarget: EventTarget::new_inherited(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBRequest> {
        reflect_dom_object(box IDBRequest::new_inherited(),
                           global,
                           IDBRequestBinding::Wrap)
    }
}
