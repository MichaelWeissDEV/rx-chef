import { test, expect, type Page } from '@playwright/test';
import { readFile, writeFile } from 'node:fs/promises';
import { createCipheriv } from 'node:crypto';
import type { Graph } from '../src/types';
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
  await page.getByRole('button', {name:'Bind empty input',exact:true}).click();
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Cancel graph', exact: true })).toBeEnabled();
  await selectNamed(page,'Selected graph node','recipe');
  await expect(page.getByLabel('Live graph node status')).toContainText('running');
  await page.getByRole('button', { name: 'Cancel graph', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText(/cancel/i);
  await expect(page.getByRole('button', { name: 'Run graph', exact: true })).toBeEnabled();
  definition.edges.push({ id: 'cycle', source_node: 'recipe', source_port: 'data', target_node: 'recipe', target_port: 'data' });
  await page.getByLabel('Graph file').setInputFiles({ name: 'invalid.rxgraph.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(definition)) });
  await expect(page.getByRole('alert')).toContainText(/duplicate connection|cycle/i);
});

for (const change of ['input', 'node', 'port']) {
  test(`stale graph results are discarded after ${change} edits`, async ({ page }) => {
    await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
    await page.getByRole('button', { name: 'Connect (this session)' }).click(); await expect(page.getByText('Operations (478)')).toBeVisible();
    await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
    await page.getByRole('button', {name:'Bind empty input',exact:true}).click();
    let release!: () => void; const gate = new Promise<void>(resolve => { release = resolve; });
    let ready!: () => void; const fetched = new Promise<void>(resolve => { ready = resolve; });
    await page.route('**/api/v1/jobs/*', async route => {
      const response = await route.fetch(); const body = await response.json();
      if (body.status === 'succeeded') { ready(); await gate; }
      await route.fulfill({ response });
    });
    await page.getByRole('button', { name: 'Run graph', exact: true }).click(); await fetched;
    if (change === 'input') await page.getByLabel('Graph input data').fill('new revision');
    else if (change === 'node') await page.getByRole('button', { name: 'Delete node', exact: true }).click();
    else { await page.getByRole('button', { name: 'Add combine', exact: true }).click(); await page.getByLabel('Combine input count').fill('3'); }
    release(); await expect(page.getByLabel('Graph run status')).toContainText('discarded');
    await expect(page.getByRole('tab')).toHaveCount(0);
  });
}

test('argument and recipe nodes can be created, connected and executed through the GUI', async ({ page }) => {
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button', { name: 'Connect (this session)' }).click(); await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await input(page, 'input', '00ffc000');
  await page.getByRole('button', { name: 'Add argument', exact: true }).click();
  await page.getByLabel('Argument node kind').selectOption('bytes');
  await connect(page, 'input', 'bytes argument', 'data');
  await page.getByRole('button', { name: 'Add recipe', exact: true }).click();
  await page.getByLabel('Recipe node operation').selectOption('to_base64'); await page.getByRole('button', { name: 'Add recipe step' }).click();
  await page.getByLabel('Recipe node operation').selectOption('from_base64'); await page.getByRole('button', { name: 'Add recipe step' }).click();
  await connect(page, 'bytes argument', 'recipe', 'data');
  await connect(page, 'recipe', 'output', 'data'); // Replaces the initial incoming output edge.
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  const download = page.waitForEvent('download'); await page.getByRole('button', { name: 'Download graph output' }).click();
  expect(await readFile((await (await download).path())!)).toEqual(Buffer.from([0, 255, 192, 0]));
});

test('AES key and IV bind as separate byte inputs with exact plaintext download', async ({ page }) => {
  const plaintext = Buffer.from([0, 255, 192, 0, 65]); const key = Buffer.alloc(16); const iv = Buffer.alloc(16, 1);
  const cipher = createCipheriv('aes-128-cbc', key, iv);
  const ciphertext = Buffer.concat([cipher.update(plaintext), cipher.final()]);
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button', {name:'Connect (this session)'}).click(); await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', {name:'Graph mode',exact:true}).click();
  await input(page, 'input', ciphertext.toString('hex'));
  for (const [name, bytes] of [['key',key],['iv',iv]] as const) {
    await page.getByRole('button', {name:'Add input',exact:true}).click(); await page.getByLabel('Node name').fill(name);
    await page.getByLabel('Graph input encoding').selectOption('hex'); await page.getByLabel('Graph input data').fill(bytes.toString('hex'));
  }
  await page.getByLabel('Graph operation',{exact:true}).selectOption('aes_decrypt');
  await page.getByRole('button', {name:'Add operation',exact:true}).click();
  await page.getByLabel('Graph argument Input',{exact:true}).selectOption('Raw');
  await connect(page,'input','AES Decrypt','data'); await connect(page,'key','AES Decrypt','arg:key'); await connect(page,'iv','AES Decrypt','arg:iv');
  await connect(page,'AES Decrypt','output','data');
  await page.getByRole('button', {name:'Run graph',exact:true}).click();
  await expect(page.getByRole('tab', {name:'output',exact:true})).toBeVisible();
  const download = page.waitForEvent('download'); await page.getByRole('button', {name:'Download graph output'}).click();
  expect(await readFile((await (await download).path())!)).toEqual(plaintext);
  await selectNamed(page,'Selected graph node','AES Decrypt');
  await expect(page.getByText('Bound from key:data',{exact:true})).toBeVisible();
  // A byte source cannot feed an enum argument. Reject without changing the graph.
  await connect(page,'key','AES Decrypt','arg:mode');
  await expect(page.getByRole('alert')).toContainText('Connection rejected');
});

test('selected outputs run only the required named input branch',async({page})=>{
  await page.goto('/');await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');await page.getByRole('button',{name:'Connect (this session)'}).click();await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button',{name:'Graph mode',exact:true}).click();await input(page,'input','00ff');
  await page.getByRole('button',{name:'Add input',exact:true}).click();await page.getByLabel('Node name').fill('secondary');
  await page.getByRole('button',{name:'Add output',exact:true}).click();await page.getByLabel('Node name').fill('unused');await connect(page,'secondary','unused','data');
  await page.getByLabel('Graph output selection').selectOption('output');await page.getByRole('button',{name:'Run graph',exact:true}).click();
  await expect(page.getByRole('tab',{name:'output',exact:true})).toBeVisible();await expect(page.getByRole('tab')).toHaveCount(1);
  const download=page.waitForEvent('download');await page.getByRole('button',{name:'Download graph output'}).click();expect(await readFile((await (await download).path())!)).toEqual(Buffer.from([0,255]));
});

test('10, 50 and 100-node graphs render and keep theme/layout controls responsive', async ({ page, context }, testInfo) => {
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button', { name: 'Connect (this session)' }).click();
  await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  const cdp = await context.newCDPSession(page); await cdp.send('Performance.enable');
  const measurements = [];
  for (const size of [10, 50, 100]) {
    const graph: Graph = { version: 1, id: `measured-${size}`, nodes: [{ id: 'input', type: 'input', name: 'input' }], edges: [], presentation: { input: { x: 0, y: 0 } } };
    const operations = Math.floor((size - 1) / 2);
    for (let i = 0; i < operations; i++) {
      graph.nodes.push({ id: `op-${i}`, type: 'operation', operation: 'to_hex', args: {}, enabled: true });
      graph.edges.push({ id: `in-${i}`, source_node: 'input', source_port: 'data', target_node: `op-${i}`, target_port: 'data' });
      graph.presentation![`op-${i}`] = { x: 300, y: i * 150 };
    }
    for (let i = 0; graph.nodes.length < size; i++) {
      graph.nodes.push({ id: `out-${i}`, type: 'output', name: `out-${i}` });
      graph.edges.push({ id: `edge-out-${i}`, source_node: `op-${i % operations}`, source_port: 'data', target_node: `out-${i}`, target_port: 'data' });
      graph.presentation![`out-${i}`] = { x: 650, y: i * 150 };
    }
    const started = performance.now();
    await page.getByLabel('Graph file', { exact: true }).setInputFiles({ name: 'measured.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(graph)) });
    await expect(page.getByLabel('Graph name', { exact: true })).toHaveValue(graph.id);
    await expect(page.locator('.react-flow__node')).toHaveCount(size);
    const imported = performance.now();
    await page.getByRole('button', { name: 'Arrange nodes', exact: true }).click();
    await page.getByTitle('Switch to light mode').click();
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    await page.getByTitle('Switch to dark mode').click();
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
    const metrics = await cdp.send('Performance.getMetrics');
    measurements.push({ nodes: size, edges: graph.edges.length, import_render_ms: imported - started,
      arrange_and_two_theme_changes_ms: performance.now() - imported,
      js_heap_used_bytes: metrics.metrics.find((metric: { name: string }) => metric.name === 'JSHeapUsedSize')?.value });
  }
  const measurementFile = testInfo.outputPath('graph-render-measurements.json');
  await writeFile(measurementFile, JSON.stringify({ platform: process.platform, architecture: process.arch, measurements }, null, 2));
  await testInfo.attach('graph-render-measurements.json', { path: measurementFile, contentType: 'application/json' });
});
