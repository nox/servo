/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSRuleListBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-cssrulelist-interface
#[dom_struct]
pub struct CSSRuleList {
    reflector_: Reflector,
}

impl CSSRuleList {
    pub fn new_inherited() -> CSSRuleList {
        CSSRuleList {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: &Window) -> Root<CSSRuleList> {
        reflect_dom_object(box CSSRuleList::new_inherited(),
                           GlobalRef::Window(global),
                           CSSRuleListBinding::Wrap)
    }
}
