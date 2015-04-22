/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSRuleBinding;
use dom::bindings::codegen::Bindings::CSSRuleBinding::CSSRuleMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-cssrule-interface
#[dom_struct]
pub struct CSSRule {
    reflector_: Reflector,
    type_: u16,
    parent_rule: Option<JS<CSSRule>>,
    parent_stylesheet: Option<JS<CSSStyleSheet>>,
}

impl CSSRule {
    pub fn new_inherited(type_: u16, parent_rule: Option<&CSSRule>,
                         parent_stylesheet: Option<&CSSStyleSheet>)
                         -> CSSRule {
        CSSRule {
            reflector_: Reflector::new(),
            type_: type_,
            parent_rule: parent_rule.map(JS::from_ref),
            parent_stylesheet: parent_stylesheet.map(JS::from_ref),
        }
    }

    pub fn new(global: &Window, type_: u16,
               parent_rule: Option<&CSSRule>,
               parent_stylesheet: Option<&CSSStyleSheet>)
               -> Root<CSSRule> {
        reflect_dom_object(box CSSRule::new_inherited(type_, parent_rule,
                                                      parent_stylesheet),
                           GlobalRef::Window(global),
                           CSSRuleBinding::Wrap)
    }

    pub fn type_(&self) -> u16 {
      self.type_
    }
}

impl CSSRuleMethods for CSSRule {
    // https://drafts.csswg.org/cssom/#dom-cssrule-type
    fn Type(&self) -> u16 {
        self.type_
    }

    // https://drafts.csswg.org/cssom/#dom-cssrule-parentrule
    fn GetParentRule(&self) -> Option<Root<CSSRule>> {
        self.parent_rule.as_ref().map(JS::root)
    }

    // https://drafts.csswg.org/cssom/#dom-cssrule-parentstylesheet
    fn GetParentStyleSheet(&self) -> Option<Root<CSSStyleSheet>> {
        self.parent_stylesheet.as_ref().map(JS::root)
    }
}
