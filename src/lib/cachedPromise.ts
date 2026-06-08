const promiseCache = new Map<string, Promise<any>>();

export function getPromise<T>(key: string, fetcher: () => Promise<T>): Promise<T> {
  if (!promiseCache.has(key)) promiseCache.set(key, fetcher());
  return promiseCache.get(key)!;
}

export function clearPromise(key: string) {
  promiseCache.delete(key);
}
