/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://w3c.github.io/IndexedDB/#idbfactory
// [Exposed=(Window,Worker)]
interface IDBFactory {
  [Throws]
  IDBOpenDBRequest open(DOMString name,
                        [EnforceRange] optional unsigned long long version);
  // IDBOpenDBRequest deleteDatabase(DOMString name);

  // short cmp(any first, any second);
};

// https://w3c.github.io/IndexedDB/#idbenvironment
[NoInterfaceObject]
interface IDBEnvironment {
  readonly attribute IDBFactory indexedDB;
};
