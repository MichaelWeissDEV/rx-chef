/** Exact byte conversions. Text previews must never become input implicitly. */
export function parseHex(input: string): Uint8Array {
  const hex = input.replace(/[ \t\r\n]/g, '');
  if (hex.length % 2 || /[^0-9a-f]/i.test(hex)) throw new Error('HEX requires complete pairs of digits (00–FF).');
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < bytes.length; i++) bytes[i] = Number.parseInt(hex.slice(i * 2, i * 2 + 2), 16);
  return bytes;
}
export function encodeHex(bytes: Uint8Array): string {
  const chunks: string[] = [];
  for (let i = 0; i < bytes.length; i += 4096) chunks.push(Array.from(bytes.subarray(i, i + 4096), b => b.toString(16).padStart(2, '0')).join(' '));
  return chunks.join(' ');
}
const alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
export function parseBase64(input: string): Uint8Array {
  const value = input.replace(/[ \t\r\n]/g, '');
  if (!value) return new Uint8Array();
  const padding = value.endsWith('==') ? 2 : value.endsWith('=') ? 1 : 0;
  const data = value.slice(0, value.length - padding);
  if (value.length % 4 || /[^A-Za-z0-9+/]/.test(data) ||
      (padding === 2 && (alphabet.indexOf(data.at(-1)!) & 15) !== 0) ||
      (padding === 1 && (alphabet.indexOf(data.at(-1)!) & 3) !== 0)) {
    throw new Error('Invalid Base64 or non-canonical padding.');
  }
  const result = new Uint8Array(value.length / 4 * 3 - padding);
  let offset = 0;
  for (let i = 0; i < value.length; i += 32768) {
    const chunk = atob(value.slice(i, i + 32768));
    for (let j = 0; j < chunk.length; j++) result[offset++] = chunk.charCodeAt(j);
  }
  return result;
}
export function encodeBase64(input: Uint8Array): string {
  const chunks: string[] = [];
  for (let i = 0; i < input.length; i += 24576) chunks.push(btoa(String.fromCharCode(...input.subarray(i, i + 24576))));
  return chunks.join('');
}
export const encodeUtf8 = (input: string): Uint8Array => new TextEncoder().encode(input);
/** Throws on invalid UTF-8. BOM bytes are retained as U+FEFF. */
export const decodeUtf8 = (input: Uint8Array): string => new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(input);
export const previewUtf8 = (input: Uint8Array): string => new TextDecoder('utf-8', { ignoreBOM: true }).decode(input);
export function parseInput(input: string, mode: 'text' | 'hex' | 'base64'): Uint8Array {
  return mode === 'hex' ? parseHex(input) : mode === 'base64' ? parseBase64(input) : encodeUtf8(input);
}
export function downloadBytes(bytes: Uint8Array, name = 'rxchef_output.bin') {
  const url = URL.createObjectURL(new Blob([bytes as BlobPart], { type: 'application/octet-stream' }));
  const link = document.createElement('a'); link.href = url; link.download = name; link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
