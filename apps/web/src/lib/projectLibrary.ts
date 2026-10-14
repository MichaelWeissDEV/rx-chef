import type { WorkbenchProject } from '../types';
import { parseContract } from './contracts';
const database = 'rxchef-projects';
const collection = 'projects';
function open(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(database, 1);
    request.onupgradeneeded = () => request.result.createObjectStore(collection, { keyPath: 'name' });
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
    request.onblocked = () => reject(new Error('Project library is blocked by another window.'));
  });
}
async function transaction<T>(mode: IDBTransactionMode, run: (store: IDBObjectStore, done: (result: T) => void, fail: (error: unknown) => void) => void): Promise<T> {
  const db = await open();
  try {
    return await new Promise<T>((resolve, reject) => {
      const tx = db.transaction(collection, mode); let result: T;
      tx.oncomplete = () => resolve(result);
      tx.onabort = () => reject(tx.error ?? new Error('Project storage transaction aborted.'));
      tx.onerror = () => reject(tx.error);
      run(tx.objectStore(collection), value => { result = value; }, error => { tx.abort(); reject(error); });
    });
  } finally { db.close(); }
}
export async function listProjects(): Promise<WorkbenchProject[]> {
  return transaction('readonly', (store, done, fail) => {
    const request = store.getAll(); request.onerror = () => fail(request.error);
    request.onsuccess = () => { try { done(request.result.map(entry => parseContract('workbench_project', entry))); } catch (error) { fail(error); } };
  });
}
export async function saveProject(project: WorkbenchProject): Promise<void> {
  return transaction('readwrite', (store, done, fail) => {
    const request = store.getAll(); request.onerror = () => fail(request.error);
    request.onsuccess = () => {
      try {
        const entries = [...request.result.filter(entry => entry.name !== project.name), project];
        const bytes = entries.reduce((total, entry) => total + new TextEncoder().encode(JSON.stringify(entry)).length, 0);
        if (entries.length > 32 || bytes > 64 * 1024 * 1024) throw new Error('Project library limit reached (32 projects / 64 MiB). Export or delete an existing project.');
        store.put(project); done(undefined);
      } catch (error) { fail(error); }
    };
  });
}
export async function deleteProject(name: string): Promise<void> {
  return transaction('readwrite', (store, done) => { store.delete(name); done(undefined); });
}
