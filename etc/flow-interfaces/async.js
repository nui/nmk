declare module async {
    declare function map<T>(arr: Array<T>, iteratee: (item: T, iterCb: Function) => void, callback: (err: any, result: Array<any>) => void): void;
    declare function each<T>(arr: Array<T>, iteratee: (item: T, iterCb: Function) => void, callback?: (err: any) => void): void;
    declare function asyncify(fn: Function): Function;
}