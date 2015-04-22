/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSStyleSheetBinding;
use dom::bindings::codegen::Bindings::CSSStyleSheetBinding::CSSStyleSheetMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::utils::reflect_dom_object;
use dom::cssrule::CSSRule;
use dom::node::Node;
use dom::stylesheet::StyleSheet;
use dom::window::Window;
use util::str::DOMString;

// https://drafts.csswg.org/cssom/#the-cssstylesheet-interface
#[dom_struct]
pub struct CSSStyleSheet {
    stylesheet: StyleSheet,
    owner_rule: Option<JS<CSSRule>>,
}

impl CSSStyleSheet {
    pub fn new_inherited(location: Option<DOMString>,
                         owner_node: Option<&Node>,
                         parent_stylesheet: Option<&StyleSheet>,
                         owner_rule: Option<&CSSRule>)
                         -> CSSStyleSheet {
        CSSStyleSheet {
            stylesheet: StyleSheet::new_inherited(location, owner_node, parent_stylesheet),
            owner_rule: owner_rule.map(JS::from_ref),
        }
    }

    pub fn new(global: &Window, location: Option<DOMString>,
               owner_node: Option<&Node>,
               parent_stylesheet: Option<&StyleSheet>,
               owner_rule: Option<&CSSRule>)
               -> Root<CSSStyleSheet> {
        reflect_dom_object(box CSSStyleSheet::new_inherited(location, owner_node,
                                                            parent_stylesheet, owner_rule),
                           GlobalRef::Window(global),
                           CSSStyleSheetBinding::Wrap)
    }
}

impl CSSStyleSheetMethods for CSSStyleSheet {
    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-ownerrule
    fn GetOwnerRule(&self) -> Option<Root<CSSRule>> {
        self.owner_rule.as_ref().map(JS::root)
    }
}
