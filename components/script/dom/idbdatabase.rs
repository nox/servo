/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBDatabaseBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::reflect_dom_object;
use dom::domexception::DOMException;
use dom::eventtarget::EventTarget;
use dom::idbopendbrequest::IDBOpenDBRequest;
use url::Origin;

#[dom_struct]
pub struct IDBDatabase {
    eventtarget: EventTarget,
}

impl IDBDatabase {
    fn new_inherited() -> IDBDatabase {
        IDBDatabase {
            eventtarget: EventTarget::new_inherited(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBDatabase> {
        reflect_dom_object(box IDBDatabase::new_inherited(),
                           global,
                           IDBDatabaseBinding::Wrap)
    }

    // https://w3c.github.io/IndexedDB/#steps-for-opening-a-database
    pub fn open(global: GlobalRef,
                origin: &Origin,
                name: &str,
                version: Option<u64>,
                request: &IDBOpenDBRequest)
                -> Result<Root<IDBDatabase>, Root<DOMException>> {
        unimplemented!()
    }
}
