"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// mod.ts
var mod_exports = {};
__export(mod_exports, {
  BiRecord: () => BiRecord,
  default: () => birecord,
  reverse: () => reverse
});
module.exports = __toCommonJS(mod_exports);
function birecord(original) {
  return new BiRecord(original);
}
var BiRecord = class {
  constructor(original, reversed = reverse(original)) {
    this.original = original;
    this.reversed = reversed;
  }
  get(key) {
    return this.original[key] ?? this.reversed[key];
  }
  has(key) {
    return key in this.original || key in this.reversed;
  }
};
function reverse(record) {
  return Object.fromEntries(
    Object.entries(record).map(([key, value]) => [value, key])
  );
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  BiRecord,
  reverse
});
