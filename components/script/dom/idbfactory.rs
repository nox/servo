/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::IDBFactoryBinding;
use dom::bindings::codegen::Bindings::IDBFactoryBinding::IDBFactoryMethods;
use dom::bindings::error::{Error, Fallible};
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::Root;
use dom::bindings::refcounted::Trusted;
use dom::bindings::reflector::{Reflectable, Reflector, reflect_dom_object};
use dom::bindings::str::DOMString;
use dom::event::{EventBubbles, EventCancelable};
use dom::eventtarget::EventTarget;
use dom::idbdatabase::IDBDatabase;
use dom::idbopendbrequest::IDBOpenDBRequest;
use dom::idbrequest::IDBRequest;
use js::conversions::ToJSValConvertible;
use js::jsval::UndefinedValue;
use script_thread::Runnable;
use task_source::TaskSource;
use task_source::database_access::DatabaseAccessTask;
use url::Origin;

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

impl IDBFactoryMethods for IDBFactory {
    // https://w3c.github.io/IndexedDB/#dom-idbfactory-open
    fn Open(&self, name: DOMString, version: Option<u64>)
            -> Fallible<Root<IDBOpenDBRequest>> {
        if version == Some(0) {
            // Step 1.
            return Err(Error::Type("Version cannot be 0".to_owned()));
        }
        // Step 2.
        let global = self.global();
        let request = IDBOpenDBRequest::new(global.r());
        // Step 3.
        let runnable = box IDBOpenRunnable {
            origin: global.r().get_url().origin(),
            name: name.into(),
            version: version,
            request: Trusted::new(&request),
        };
        global.r()
            .database_access_task_source()
            .queue(DatabaseAccessTask::Runnable(runnable))
            .unwrap();
        // Step 4.
        Ok(request)
    }
}

struct IDBOpenRunnable {
    origin: Origin,
    name: String,
    version: Option<u64>,
    request: Trusted<IDBOpenDBRequest>,
}

impl Runnable for IDBOpenRunnable {
    fn name(&self) -> &'static str { "IDBOpenRunnable" }

    // https://w3c.github.io/IndexedDB/#dom-idbfactory-open, step 3.
    #[allow(unsafe_code)]
    fn handler(self: Box<IDBOpenRunnable>) {
        let request = self.request.root();
        match IDBDatabase::open(&self.origin, &self.name, self.version, &request) {
            Ok(db) => {
                // Step 3.3.
                let global = request.global();
                let cx = global.r().get_cx();
                rooted!(in(cx) let mut result = UndefinedValue());
                unsafe {
                    db.to_jsval(cx, result.handle_mut());
                }
                request.upcast::<IDBRequest>().set_result(result.handle());
                request.upcast::<EventTarget>().fire_event(
                    "success", EventBubbles::DoesNotBubble, EventCancelable::NotCancelable);
            },
            Err(error) => {
                // Step 3.2.
                request.upcast::<IDBRequest>().set_error(&error);
                request.upcast::<EventTarget>().fire_event(
                    "error", EventBubbles::Bubbles, EventCancelable::NotCancelable);
            }
        }
    }
}
