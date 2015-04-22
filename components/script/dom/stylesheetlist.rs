/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StyleSheetListBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::node::Node;
use dom::stylesheet::StyleSheet;
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-stylesheetlist-interface
#[dom_struct]
pub struct StyleSheetList {
    reflector_: Reflector,
    owner: JS<Node>,
    items: Vec<JS<StyleSheet>>,
}

impl StyleSheetList {
    pub fn new_inherited(owner: &Node) -> StyleSheetList {
        StyleSheetList {
            reflector_: Reflector::new(),
            owner: JS::from_ref(owner),
            items: vec![],
        }
    }

    pub fn new(global: &Window, owner: &Node)
               -> Root<StyleSheetList> {
        reflect_dom_object(box StyleSheetList::new_inherited(owner),
                           GlobalRef::Window(global),
                           StyleSheetListBinding::Wrap)
    }
}
