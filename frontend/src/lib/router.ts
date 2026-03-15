import { writable } from 'svelte/store';

function getInitialLocation(): string {
  if (typeof window === 'undefined') return '/';
  return window.location.hash.slice(1) || '/';
}

export const location = writable(getInitialLocation());

if (typeof window !== 'undefined') {
  window.addEventListener('hashchange', () => {
    location.set(window.location.hash.slice(1) || '/');
  });
}

export function navigate(path: string) {
  if (typeof window !== 'undefined') {
    window.location.hash = path;
  }
}

export const params = writable<Record<string, string>>({});

export function getParams(): Record<string, string> {
  let currentLocation = '';
  location.subscribe(v => currentLocation = v)();
  const result: Record<string, string> = {};
  const pathParts = currentLocation.split('/');
  return result;
}
