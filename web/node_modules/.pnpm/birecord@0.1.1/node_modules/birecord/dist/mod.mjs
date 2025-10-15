// mod.ts
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
export {
  BiRecord,
  birecord as default,
  reverse
};
