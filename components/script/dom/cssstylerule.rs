/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSStyleRuleBinding;
use dom::bindings::codegen::Bindings::CSSRuleBinding::CSSRuleConstants;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::reflect_dom_object;
use dom::cssrule::CSSRule;
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-cssstylerule-interface
#[dom_struct]
pub struct CSSStyleRule {
    rule: CSSRule,
}

impl CSSStyleRule {
    pub fn new_inherited(parent_rule: Option<&CSSRule>,
                         parent_stylesheet: Option<&CSSStyleSheet>)
                         -> CSSStyleRule {
        CSSStyleRule {
            rule: CSSRule::new_inherited(CSSRuleConstants::STYLE_RULE,
                                         parent_rule, parent_stylesheet),
        }
    }

    pub fn new(global: &Window, parent_rule: Option<&CSSRule>,
               parent_stylesheet: Option<&CSSStyleSheet>)
               -> Root<CSSStyleRule> {
        reflect_dom_object(box CSSStyleRule::new_inherited(parent_rule,
                                                           parent_stylesheet),
                           GlobalRef::Window(global),
                           CSSStyleRuleBinding::Wrap)
    }
}
