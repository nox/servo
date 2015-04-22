/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::CSSMediaRuleBinding;
use dom::bindings::codegen::Bindings::CSSRuleBinding::CSSRuleConstants;
use dom::bindings::codegen::InheritTypes::CSSMediaRuleDerived;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::Root;
use dom::bindings::utils::reflect_dom_object;
use dom::cssgroupingrule::CSSGroupingRule;
use dom::cssrule::CSSRule;
use dom::cssstylesheet::CSSStyleSheet;
use dom::window::Window;

// https://drafts.csswg.org/cssom/#the-cssmediarule-interface
#[dom_struct]
pub struct CSSMediaRule {
    rule: CSSGroupingRule,
}

impl CSSMediaRuleDerived for CSSRule {
  fn is_cssmediarule(&self) -> bool {
    self.type_() == CSSRuleConstants::MEDIA_RULE
  }
}

impl CSSMediaRule {
    pub fn new_inherited(parent_rule: Option<&CSSRule>,
                         parent_stylesheet: Option<&CSSStyleSheet>)
                         -> CSSMediaRule {
        CSSMediaRule {
            rule: CSSGroupingRule::new_inherited(CSSRuleConstants::MEDIA_RULE,
                                                 parent_rule, parent_stylesheet),
        }
    }

    pub fn new(global: &Window, parent_rule: Option<&CSSRule>,
               parent_stylesheet: Option<&CSSStyleSheet>)
               -> Root<CSSMediaRule> {
        reflect_dom_object(box CSSMediaRule::new_inherited(parent_rule,
                                                           parent_stylesheet),
                           GlobalRef::Window(global),
                           CSSMediaRuleBinding::Wrap)
    }
}
