import { writable, get, derived } from 'svelte/store';
import { translations, type Locale, type Translations } from './translations';

export const locale = writable<Locale>('es');

export function t(key: keyof Translations): string {
  return translations[get(locale)][key] || key;
}

export const localeSubscribe = {
  subscribe: locale.subscribe
};

export const tStore = derived(locale, ($locale) => {
  return (key: keyof Translations): string => {
    return translations[$locale][key] || key;
  };
});

export function setLocale(newLocale: Locale): void {
  locale.set(newLocale);
}

export function getLocale(): Locale {
  return get(locale);
}

export function toggleLocale(): void {
  const current = get(locale);
  locale.set(current === 'es' ? 'en' : 'es');
}
