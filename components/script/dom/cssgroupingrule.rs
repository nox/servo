/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSGroupingRuleBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::reflect_dom_object;
use dom::cssrule::CSSRule;
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-cssgroupingrule-interface
#[dom_struct]
pub struct CSSGroupingRule {
    rule: CSSRule,
}

impl CSSGroupingRule {
    pub fn new_inherited(type_: u16, parent_rule: Option<&CSSRule>,
                         parent_stylesheet: Option<&CSSStyleSheet>)
                         -> CSSGroupingRule {
        CSSGroupingRule {
            rule: CSSRule::new_inherited(type_, parent_rule, parent_stylesheet),
        }
    }

    pub fn new(global: &Window, type_: u16,
               parent_rule: Option<&CSSRule>,
               parent_stylesheet: Option<&CSSStyleSheet>)
               -> Root<CSSGroupingRule> {
        reflect_dom_object(box CSSGroupingRule::new_inherited(type_, parent_rule,
                                                              parent_stylesheet),
                           GlobalRef::Window(global),
                           CSSGroupingRuleBinding::Wrap)
    }
}
