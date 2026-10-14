import { describe, expect, it } from 'vitest';
import { parseHex, encodeHex, parseBase64, encodeBase64, encodeUtf8, decodeUtf8, previewUtf8 } from './bytes';

describe('exact bytes', () => {
  const all = Uint8Array.from({ length: 256 }, (_, i) => i);
  for (const bytes of [new Uint8Array(), all, new Uint8Array([0, 255, 192, 128, 0]), Uint8Array.from({ length: 2 * 1024 * 1024 }, (_, i) => i % 256)]) {
    it(`round trips ${bytes.length} bytes without text conversion`, () => {
      expect(parseBase64(encodeBase64(bytes))).toEqual(bytes);
      expect(parseHex(encodeHex(bytes))).toEqual(bytes);
    });
  }
  it.each(['0', 'GG', 'FF 0Z', '0xFF', '00\u200bff'])('rejects malformed HEX %s', input => expect(() => parseHex(input)).toThrow());
  it.each(['A', 'AA', 'A===', 'AAA==', '====', 'AA=A', 'AB==', 'AAB=', 'AA-_'])('rejects malformed Base64 %s', input => expect(() => parseBase64(input)).toThrow());
  it('accepts complete HEX pairs and ASCII whitespace', () => {
    expect(parseHex('00 ff 7a')).toEqual(new Uint8Array([0, 255, 122]));
    expect(parseHex('00FF7A')).toEqual(parseHex('00 ff 7a'));
    expect(parseBase64('AP8=\n')).toEqual(new Uint8Array([0, 255]));
  });
  it('separates strict UTF-8 and lossy preview', () => {
    expect(() => decodeUtf8(all)).toThrow();
    expect(previewUtf8(all)).toContain('\uFFFD');
    expect(encodeUtf8(decodeUtf8(encodeUtf8('\uFEFFGrüße\0')))).toEqual(encodeUtf8('\uFEFFGrüße\0'));
  });
});
