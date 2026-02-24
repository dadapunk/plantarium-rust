import { writable, derived } from 'svelte/store';

export const location = writable(window.location.hash.slice(1) || '/');

window.addEventListener('hashchange', () => {
  location.set(window.location.hash.slice(1) || '/');
});

export function navigate(path: string) {
  window.location.hash = path;
}

export const params = writable<Record<string, string>>({});

export function getParams(): Record<string, string> {
  let currentLocation = '';
  location.subscribe(v => currentLocation = v)();
  
  const result: Record<string, string> = {};
  const pathParts = currentLocation.split('/');
  
  return result;
}
