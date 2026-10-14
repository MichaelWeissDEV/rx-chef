import { test, expect, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';
async function selectNamed(page: Page, label: string, name: string) {
  const select = page.getByLabel(label, { exact: true });
  const value = await select.locator('option').filter({ hasText: new RegExp(`^${name} ·`) }).first().getAttribute('value');
  await select.selectOption(value!); return value!;
}
async function connect(page: Page, source: string, target: string, port: string) {
  await selectNamed(page, 'Connection source', source); await selectNamed(page, 'Connection target', target);
  await page.getByLabel('Connection port').selectOption(port); await page.getByRole('button', { name: 'Connect nodes', exact: true }).click();
}
async function input(page: Page, name: string, value: string, mode = 'hex') {
  await selectNamed(page, 'Selected graph node', name);
  await page.getByLabel('Graph input encoding').selectOption(mode); await page.getByLabel('Graph input data').fill(value);
}
test('build and run three-input graph with fan-out and three exact named downloads', async ({ page }) => {
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button', { name: 'Connect (this session)' }).click();
  await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  const bytes = Buffer.from(Array.from({ length: 256 }, (_, i) => i)); const hex = bytes.toString('hex');
  await input(page, 'input', hex);
  for (const name of ['second', 'third']) {
    await page.getByRole('button', { name: 'Add input', exact: true }).click(); await page.getByLabel('Node name').fill(name);
    await page.getByLabel('Graph input encoding').selectOption('hex'); await page.getByLabel('Graph input data').fill(name === 'second' ? '00ff' : 'c000');
  }
  await page.getByRole('button', { name: 'Add combine', exact: true }).click(); await page.getByLabel('Combine input count').fill('3');
  await connect(page, 'input', 'concatenate', 'input:0'); await connect(page, 'second', 'concatenate', 'input:1'); await connect(page, 'third', 'concatenate', 'input:2');
  await page.getByRole('button', { name: 'Add output', exact: true }).click(); await page.getByLabel('Node name').fill('combined');
  await connect(page, 'concatenate', 'combined', 'data');
  await page.getByRole('button', { name: 'Add operation', exact: true }).click();
  await connect(page, 'input', 'To Base64', 'data');
  await page.getByRole('button', { name: 'Add output', exact: true }).click(); await page.getByLabel('Node name').fill('encoded');
  await connect(page, 'To Base64', 'encoded', 'data');
  await page.getByRole('button', { name: 'Arrange nodes', exact: true }).click();
  await page.getByRole('button', { name: /fit view/i }).click();
  await expect.poll(() => page.locator('.react-flow__node').evaluateAll(nodes => nodes.every(node => {
    const rect = node.getBoundingClientRect(); const canvas = node.closest('.graph-canvas')!.getBoundingClientRect();
    return rect.left >= canvas.left - 1 && rect.right <= canvas.right + 1 && rect.top >= canvas.top - 1 && rect.bottom <= canvas.bottom + 1;
  }))).toBe(true);
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'combined', exact: true })).toBeVisible();
  for (const [name, expected] of [['output', bytes], ['combined', Buffer.concat([bytes, Buffer.from([0, 255, 192, 0])])], ['encoded', Buffer.from(bytes.toString('base64'))]] as const) {
    await page.getByRole('tab', { name, exact: true }).click();
    const download = page.waitForEvent('download'); await page.getByRole('button', { name: 'Download graph output' }).click();
    expect(await readFile((await (await download).path())!)).toEqual(expected);
  }
  await page.getByRole('button', { name: 'List mode', exact: true }).click();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'combined', exact: true })).toBeVisible();
  const download = page.waitForEvent('download'); await page.getByRole('button', { name: 'Save graph', exact: true }).click();
  const saved = await readFile((await (await download).path())!); expect(saved.toString()).not.toContain(hex);
  await page.getByLabel('Graph file').setInputFiles({ name: 'saved.rxgraph.json', mimeType: 'application/json', buffer: saved });
  await expect(page.getByRole('status')).toHaveText('Graph loaded. Bind input values before running.');
  await input(page, 'input', hex); await input(page, 'second', '00ff'); await input(page, 'third', 'c000');
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'combined', exact: true })).toBeVisible();
  await page.getByRole('tab', { name: 'combined', exact: true }).click();
  const reloaded = page.waitForEvent('download'); await page.getByRole('button', { name: 'Download graph output' }).click();
  expect(await readFile((await (await reloaded).path())!)).toEqual(Buffer.concat([bytes, Buffer.from([0, 255, 192, 0])]));
  await page.screenshot({ path: '/private/tmp/rxchef-graph-browser.png', fullPage: true });
});

test('graph import validation and cancellation use the real supervised server', async ({ page }) => {
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button', { name: 'Connect (this session)' }).click(); await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  const definition = { version: 1, id: 'slow', nodes: [
    { id: 'input', type: 'input', name: 'input' },
    { id: 'recipe', type: 'recipe', steps: [{ op: 'Derive PBKDF2 key', args: ['password', '128', '4000000000', 'SHA256', 'salt'] }] },
    { id: 'output', type: 'output', name: 'output' }], edges: [
    { id: 'a', source_node: 'input', source_port: 'data', target_node: 'recipe', target_port: 'data' },
    { id: 'b', source_node: 'recipe', source_port: 'data', target_node: 'output', target_port: 'data' }] };
  await page.getByLabel('Graph file').setInputFiles({ name: 'slow.rxgraph.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(definition)) });
  await expect(page.getByRole('status')).toHaveText('Graph loaded. Bind input values before running.');
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Cancel graph', exact: true })).toBeEnabled();
  await page.getByRole('button', { name: 'Cancel graph', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText(/cancel/i);
  await expect(page.getByRole('button', { name: 'Run graph', exact: true })).toBeEnabled();
  definition.edges.push({ id: 'cycle', source_node: 'recipe', source_port: 'data', target_node: 'recipe', target_port: 'data' });
  await page.getByLabel('Graph file').setInputFiles({ name: 'invalid.rxgraph.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(definition)) });
  await expect(page.getByRole('alert')).toContainText(/duplicate connection|cycle/i);
});
