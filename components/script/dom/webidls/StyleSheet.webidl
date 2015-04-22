/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://drafts.csswg.org/cssom/#the-stylesheet-interface
 */

interface StyleSheet {
  readonly attribute DOMString type;
  readonly attribute DOMString? href;
  readonly attribute Node /* (Element or ProcessingInstruction) */ ? ownerNode;
  readonly attribute StyleSheet? parentStyleSheet;
  // readonly attribute DOMString? title;
  // [SameObject /*, PutForwards=mediaText */]
  // readonly attribute MediaList media;
  readonly
  attribute boolean disabled;
};
