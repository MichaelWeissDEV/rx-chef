/** Preferences are optional and may contain stale or externally edited values. */
export function readPreferenceList(key: string, fallback: string[] = []): string[] {
  try {
    const stored = localStorage.getItem(key);
    if (stored === null) return fallback;
    const value: unknown = JSON.parse(stored);
    return Array.isArray(value) ? [...new Set(value.filter((entry): entry is string => typeof entry === 'string'))].slice(0, 1000) : fallback;
  } catch { return fallback; }
}
