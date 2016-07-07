/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBFactoryBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};

#[dom_struct]
pub struct IDBFactory {
    reflector_: Reflector,
}

impl IDBFactory {
    fn new_inherited() -> IDBFactory {
        IDBFactory {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBFactory> {
        reflect_dom_object(box IDBFactory::new_inherited(),
                           global,
                           IDBFactoryBinding::Wrap)
    }
}
