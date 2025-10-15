declare function birecord<const T extends Record<any, any>>(original: T): BiRecord<T>;
declare class BiRecord<const T extends Record<any, any>> {
    original: T;
    reversed: Reverse<T>;
    constructor(original: T, reversed?: Reverse<T>);
    get<U extends keyof T | T[keyof T]>(key: U): U extends keyof T ? T[U] : U extends T[keyof T] ? Reverse<T>[U] : unknown;
    has(key: any): key is keyof T | T[keyof T];
}
declare function reverse<T extends Record<any, any>>(record: T): Reverse<T>;
type Reverse<T extends Record<any, any>> = {
    [U in keyof T as T[U]]: U;
};

export { BiRecord, Reverse, birecord as default, reverse };
