/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBRequestBinding;
use dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestMethods;
use dom::bindings::codegen::Bindings::IDBRequestBinding::IDBRequestReadyState;
use dom::bindings::error::{Error, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, MutHeapJSVal, MutNullableHeap, Root};
use dom::bindings::reflector::reflect_dom_object;
use dom::domexception::DOMException;
use dom::eventtarget::EventTarget;
use js::jsapi::{HandleValue, JSContext};
use js::jsval::JSVal;
use std::cell::Cell;

#[dom_struct]
pub struct IDBRequest {
    eventtarget: EventTarget,
    #[ignore_heap_size_of = "JSVal is hard"]
    result: MutHeapJSVal,
    error: MutNullableHeap<JS<DOMException>>,
    state: Cell<IDBRequestReadyState>,
}

impl IDBRequest {
    pub fn new_inherited() -> IDBRequest {
        IDBRequest {
            eventtarget: EventTarget::new_inherited(),
            result: MutHeapJSVal::new(),
            error: MutNullableHeap::new(None),
            state: Cell::new(IDBRequestReadyState::Pending),
        }
    }

    pub fn new(global: GlobalRef) -> Root<IDBRequest> {
        reflect_dom_object(box IDBRequest::new_inherited(),
                           global,
                           IDBRequestBinding::Wrap)
    }

    pub fn set_result(&self, result: HandleValue) {
        assert_eq!(self.state.get(), IDBRequestReadyState::Pending);
        self.result.set(result.get());
        self.state.set(IDBRequestReadyState::Done);
    }

    pub fn set_error(&self, error: &DOMException) {
        assert_eq!(self.state.get(), IDBRequestReadyState::Pending);
        self.error.set(Some(error));
        self.state.set(IDBRequestReadyState::Done);
    }
}

impl IDBRequestMethods for IDBRequest {
    // https://w3c.github.io/IndexedDB/#dom-idbrequest-result
    fn GetResult(&self, _: *mut JSContext) -> Fallible<JSVal> {
        if self.state.get() == IDBRequestReadyState::Pending {
            return Err(Error::InvalidState);
        }
        Ok(self.result.get())
    }

    // https://w3c.github.io/IndexedDB/#dom-idbrequest-error
    fn GetError(&self) -> Fallible<Option<Root<DOMException>>> {
        if self.state.get() == IDBRequestReadyState::Pending {
            return Err(Error::InvalidState);
        }
        Ok(self.error.get())
    }

    // https://w3c.github.io/IndexedDB/#dom-idbrequest-readystate
    fn ReadyState(&self) -> IDBRequestReadyState {
        self.state.get()
    }

    event_handler!(success, GetOnsuccess, SetOnsuccess);
    event_handler!(error, GetOnerror, SetOnerror);
}
