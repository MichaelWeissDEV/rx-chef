import { afterEach, expect, it, vi } from 'vitest';
import { readPreferenceList } from './preferences';
afterEach(() => vi.unstubAllGlobals());
it('ignores stale non-array preferences and malformed JSON', () => {
  for (const stored of ['{}', 'null', '{broken']) {
    vi.stubGlobal('localStorage', { getItem: () => stored });
    expect(readPreferenceList('favorites', ['to_base64'])).toEqual(['to_base64']);
  }
});
it('filters mixed values, deduplicates entries and respects an intentionally empty list', () => {
  vi.stubGlobal('localStorage', { getItem: () => '["to_base64",1,null,"to_base64"]' });
  expect(readPreferenceList('favorites')).toEqual(['to_base64']);
  vi.stubGlobal('localStorage', { getItem: () => '[]' });
  expect(readPreferenceList('favorites', ['to_base64'])).toEqual([]);
});
it('keeps the UI usable when storage access is denied', () => {
  vi.stubGlobal('localStorage', { getItem: () => { throw new Error('denied'); } });
  expect(readPreferenceList('recent')).toEqual([]);
});
