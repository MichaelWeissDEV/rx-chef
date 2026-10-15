import { test, expect, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';
import { starterGraph } from '../src/features/graph/model';
const token = 'browser-test-token-0123456789';
async function ready(page: Page) {
  await page.goto('/'); await page.getByLabel('Server Bearer Token').fill(token);
  await page.getByRole('button', { name: 'Connect (this session)', exact: true }).click();
  await expect(page.getByText('Operations (478)')).toBeVisible();
}
function shareProject() {
  return { version: 1, name: 'Team workspace', graphs: [{ ...starterGraph(), id: 'z' }, { ...starterGraph(), id: 'a' }],
    recipes: [{ name: 'Recipe A', steps: [{ op: 'To Base64', args: [], enabled: true, breakpoint: false }] }, { name: 'Recipe B', steps: [] }],
    settings: { theme: 'dark', selected_graph: 'z', selected_recipe: 'Recipe A' },
    input_files: [{ graph_id: 'z', input_name: 'input', path: '/private/team/source.bin', external: true }],
    history: [{ id: 'old-run', run_id: 'run-1', graph_id: 'z', started_at: '2026-10-15T00:00:00Z', status: 'succeeded', elapsed_ms: 1, output_sizes: { output: 3 } }] };
}
test('guided graph builds a real byte chain and layout edits preserve exact results', async ({ page }, info) => {
  await ready(page); await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await expect(page.getByText('Connect your data, step by step', { exact: true })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Add argument', exact: true })).not.toBeVisible();
  await page.getByRole('button', { name: 'Bind input input →', exact: true }).click();
  await page.getByLabel('Graph input data').fill('Hello');
  await page.getByLabel('Search graph operations').fill('To Base64');
  await page.getByRole('button', { name: 'Add graph operation To Base64', exact: true }).click();
  await expect(page.locator('.react-flow__node')).toHaveCount(3);
  await expect(page.locator('.react-flow__edge')).toHaveCount(2);
  await page.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Arrange nodes', exact: true }).click();
  await expect(page.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  await page.getByLabel('Selected graph node').selectOption('input');
  await page.getByLabel('Node display label').fill('Readable input');
  await expect(page.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  const input = page.locator('.react-flow__node[data-id="input"] strong'); const box = (await input.boundingBox())!;
  await page.mouse.move(box.x + 15, box.y + 8); await page.mouse.down(); await page.mouse.move(box.x + 45, box.y + 30, { steps: 8 }); await page.mouse.up();
  await expect(page.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  const download = page.waitForEvent('download'); await page.getByRole('button', { name: 'Download graph output' }).click();
  expect(await readFile((await (await download).path())!)).toEqual(Buffer.from('SGVsbG8='));
  await page.screenshot({ path: info.outputPath('guided-workbench.png'), fullPage: true });
});

test('late graph import cannot replace a graph edited during validation', async ({ page }) => {
  await ready(page); await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  let release!: () => void; let received!: () => void;
  const gate = new Promise<void>(resolve => { release = resolve; }); const pending = new Promise<void>(resolve => { received = resolve; });
  await page.route('**/api/v1/graphs/validate', async route => { const response = await route.fetch(); received(); await gate; await route.fulfill({ response }); });
  await page.getByLabel('Graph file', { exact: true }).setInputFiles({ name: 'old.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify({ ...starterGraph(), id: 'old-import' })) });
  await pending; await page.getByLabel('Graph name', { exact: true }).fill('keep-my-edit'); release();
  await expect(page.getByRole('alert')).toContainText('Graph import discarded');
  await expect(page.getByLabel('Graph name', { exact: true })).toHaveValue('keep-my-edit');
});

test('selective Git export reviews imports and roundtrips through the shared UI', async ({ page, context }, info) => {
  await ready(page); await page.locator('.project-panel > summary').click();
  const source = { name: 'team.rxproject.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(shareProject())) };
  await page.getByLabel('Project file', { exact: true }).setInputFiles(source);
  await expect(page.getByRole('dialog', { name: 'Open Team workspace?' })).toBeVisible();
  await page.getByRole('button', { name: 'Cancel import', exact: true }).click();
  await expect(page.getByLabel('Project name', { exact: true })).toHaveValue('Workspace');
  await page.getByLabel('Project file', { exact: true }).setInputFiles(source);
  await page.getByRole('button', { name: 'Open imported project', exact: true }).click();
  await expect(page.getByLabel('Project name', { exact: true })).toHaveValue('Team workspace');
  await page.getByRole('button', { name: 'Export project', exact: true }).click();
  const dialog = page.getByRole('dialog', { name: 'Export & share project' });
  await expect(dialog.getByLabel('Run history', { exact: true })).not.toBeChecked();
  await expect(dialog.getByLabel('External file references', { exact: true })).not.toBeChecked();
  await dialog.getByRole('checkbox', { name: 'z', exact: true }).uncheck();
  await dialog.getByLabel('Node layout', { exact: true }).uncheck();
  await dialog.getByText('Preview export JSON', { exact: true }).click();
  await expect(dialog.getByLabel('Project export preview')).toContainText('Team workspace');
  const screenshot = info.outputPath('selective-export.png'); await page.screenshot({ path: screenshot });
  const download = page.waitForEvent('download'); await dialog.getByRole('button', { name: 'Download project file', exact: true }).click();
  const file = await download; expect(file.suggestedFilename()).toBe('Team-workspace.rxproject.json');
  const bytes = await readFile((await file.path())!); const exported = JSON.parse(bytes.toString());
  expect(exported.graphs.map((graph: { id: string }) => graph.id)).toEqual(['a']);
  expect(exported.recipes).toHaveLength(2); expect(exported.graphs[0].presentation).toEqual({});
  expect(exported.history).toEqual([]); expect(exported.input_files).toEqual([]); expect(exported.settings).toEqual({});
  expect(bytes.toString()).not.toContain('/private/team'); expect(bytes.toString()).not.toContain(token); expect(bytes.toString().endsWith('\n')).toBe(true);
  const other = await context.newPage(); await ready(other); await other.locator('.project-panel > summary').click();
  await other.getByLabel('Project file', { exact: true }).setInputFiles({ name: file.suggestedFilename(), mimeType: 'application/json', buffer: bytes });
  await other.getByRole('button', { name: 'Open imported project', exact: true }).click();
  await other.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await expect(other.getByLabel('Graph name', { exact: true })).toHaveValue('a');
  await expect(other.getByLabel('Graph input data')).toHaveValue('');
  await other.getByLabel('Graph input data').fill('second client');
  await other.getByRole('button', { name: 'Run graph', exact: true }).click();
  await expect(other.getByRole('tab', { name: 'output', exact: true })).toBeVisible();
  const output = other.waitForEvent('download'); await other.getByRole('button', { name: 'Download graph output' }).click();
  expect(await readFile((await (await output).path())!)).toEqual(Buffer.from('second client'));
});

test('native export cancellation keeps the export open and uses a project filename', async ({ page }) => {
  await page.addInitScript(({ token }) => {
    Object.assign(window, { __TAURI_INTERNALS__: { invoke: async (command: string, args: { method?: string; params?: unknown; filename?: string }) => {
      if (command === 'export_file') { Object.assign(window, { lastExportFilename: args.filename }); return false; }
      if (command !== 'engine_call') throw new Error('Unexpected bridge call');
      const method = args.method; const path = method === 'operations' ? '/operations' : method === 'capabilities' ? '/capabilities' : '/projects/validate';
      const response = await fetch('/api/v1' + path, { method: method === 'project_validate' ? 'POST' : 'GET', headers: { Authorization: 'Bearer ' + token, 'Content-Type': 'application/json' }, ...(method === 'project_validate' ? { body: JSON.stringify(args.params) } : {}) });
      if (!response.ok) throw await response.json(); return response.json();
    } } });
  }, { token });
  await page.goto('/'); await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.getByRole('button', { name: 'Export project', exact: true }).click();
  const dialog = page.getByRole('dialog', { name: 'Export & share project' });
  await dialog.getByRole('button', { name: 'Download project file', exact: true }).click();
  await expect(dialog.getByRole('button', { name: 'Download project file', exact: true })).toBeEnabled();
  await expect(dialog).toBeVisible();
  expect(await page.evaluate(() => (window as unknown as { lastExportFilename: string }).lastExportFilename)).toBe('Workspace.rxproject.json');
  await expect(page.getByText('Project exported. External files are references and are not included.', { exact: true })).toHaveCount(0);
});

test('project import cannot overwrite edits made while validation is pending', async ({ page }) => {
  await ready(page); await page.locator('.project-panel > summary').click();
  let release!: () => void; let received!: () => void;
  const gate = new Promise<void>(resolve => { release = resolve; }); const pending = new Promise<void>(resolve => { received = resolve; });
  await page.route('**/api/v1/projects/validate', async route => { const response = await route.fetch(); received(); await gate; await route.fulfill({ response }); });
  await page.getByLabel('Project file', { exact: true }).setInputFiles({ name: 'team.json', mimeType: 'application/json', buffer: Buffer.from(JSON.stringify(shareProject())) });
  await pending; await page.getByLabel('Project name', { exact: true }).fill('Keep current work'); release();
  await expect(page.locator('.project-panel').getByRole('alert')).toContainText('Project import discarded');
  await expect(page.getByLabel('Project name', { exact: true })).toHaveValue('Keep current work');
  await expect(page.getByRole('dialog')).toHaveCount(0);
});

test('export is keyboard dismissible, prevents empty selection and fits a narrow window', async ({ page }) => {
  await ready(page); await page.setViewportSize({ width: 390, height: 844 });
  await page.getByRole('button', { name: 'Export project', exact: true }).click();
  const dialog = page.getByRole('dialog', { name: 'Export & share project' });
  const definitions = dialog.getByRole('group', { name: 'Definitions' });
  for (const checkbox of await definitions.getByRole('checkbox').all()) await checkbox.uncheck();
  await expect(dialog.getByRole('button', { name: 'Download project file', exact: true })).toBeDisabled();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  const bounds = (await dialog.boundingBox())!; expect(bounds.x).toBeGreaterThanOrEqual(0); expect(bounds.x + bounds.width).toBeLessThanOrEqual(390);
  await page.keyboard.press('Escape'); await expect(dialog).toHaveCount(0);
});

test('operation search ranks exact names first and favorites stay shared between modes', async ({ page }) => {
  await page.addInitScript(() => { localStorage.setItem('rxchef_favs', '{}'); localStorage.setItem('rxchef_recent_ops', '{broken'); });
  await ready(page); await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await page.getByLabel('Search graph operations').fill('To Base64');
  await expect(page.locator('.graph-operation-row').first().getByRole('button', { name: 'Add graph operation To Base64', exact: true })).toBeVisible();
  const favorite = page.getByRole('button', { name: 'Favorite To Base64', exact: true });
  const previous = await favorite.getAttribute('aria-pressed'); await favorite.click();
  await page.getByRole('button', { name: 'List mode', exact: true }).click();
  await page.getByRole('button', { name: 'Graph mode', exact: true }).click();
  await expect(favorite).toHaveAttribute('aria-pressed', previous === 'true' ? 'false' : 'true');
});
