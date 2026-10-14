import { test, expect, type Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';
async function connect(page:Page) {
  await page.goto('/');await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');
  await page.getByRole('button',{name:'Connect (this session)'}).click();await expect(page.getByText('Operations (478)')).toBeVisible();
  await page.locator('.project-panel > summary').click();
}
test('a multi-graph project survives reload with metadata history and explicitly rebound exact bytes',async({page})=>{
  await connect(page);await page.getByLabel('Project name').fill('Saved session');
  await page.getByRole('button',{name:'Graph mode',exact:true}).click();await page.getByLabel('Graph name',{exact:true}).fill('first');
  await page.getByLabel('Graph input data').fill('PRIVATE-INPUT-DO-NOT-SAVE');
  await page.getByRole('button',{name:'New project graph',exact:true}).click();await page.getByLabel('Graph name',{exact:true}).fill('second');
  const bytes=Buffer.from(Array.from({length:256},(_,i)=>i));
  await page.getByLabel('Graph input encoding').selectOption('hex');await page.getByLabel('Graph input data').fill(bytes.toString('hex'));
  await page.getByRole('button',{name:'Run graph',exact:true}).click();await expect(page.getByRole('tab',{name:'output',exact:true})).toBeVisible();
  await page.getByRole('button',{name:'Save project locally',exact:true}).click();
  await expect(page.getByText('Project saved locally without input payloads or credentials.',{exact:true})).toBeVisible();
  const downloaded=page.waitForEvent('download');await page.getByRole('button',{name:'Export project',exact:true}).click();
  const text=(await readFile((await (await downloaded).path())!)).toString();const project=JSON.parse(text);
  expect(project.graphs.map((graph:{id:string})=>graph.id)).toEqual(['first','second']);
  expect(project.history).toHaveLength(1);expect(project.history[0].output_sizes.output).toBe(256);
  expect(project.history[0].status).toBe('succeeded');expect(project.recipes[0].steps[0].op).toBe('To Base64');
  for(const secret of ['PRIVATE-INPUT-DO-NOT-SAVE',bytes.toString('hex'),'browser-test-token-0123456789','output_base64','inputs_base64'])expect(text).not.toContain(secret);
  await page.reload();await page.getByLabel('Server Bearer Token').fill('browser-test-token-0123456789');await page.getByRole('button',{name:'Connect (this session)'}).click();
  await expect(page.getByText('Operations (478)')).toBeVisible();await page.locator('.project-panel > summary').click();
  await page.getByLabel('Saved projects',{exact:true}).selectOption('Saved session');await page.getByRole('button',{name:'Open saved project',exact:true}).click();
  await expect(page.getByText('Project restored. Inputs are unbound; select files or enter values again.',{exact:true})).toBeVisible();
  await page.getByRole('button',{name:'Graph mode',exact:true}).click();await expect(page.getByLabel('Graph name',{exact:true})).toHaveValue('second');
  await expect(page.getByLabel('Graph input data')).toHaveValue('');await page.getByRole('button',{name:'Run graph',exact:true}).click();
  await expect(page.getByRole('alert')).toContainText('Bind input input');
  await page.getByLabel('Graph input encoding').selectOption('hex');await page.getByLabel('Graph input data').fill(bytes.toString('hex'));
  await page.getByRole('button',{name:'Run graph',exact:true}).click();await expect(page.getByRole('tab',{name:'output',exact:true})).toBeVisible();
  const output=page.waitForEvent('download');await page.getByRole('button',{name:'Download graph output'}).click();
  expect(await readFile((await (await output).path())!)).toEqual(bytes);
  await page.getByLabel('Project graphs',{exact:true}).selectOption('first');await expect(page.getByLabel('Graph name',{exact:true})).toHaveValue('first');
  await expect(page.getByLabel('Graph input data')).toHaveValue('');
});
test('project import rejects unknown credentials and sensitive recipe literals without exposing values',async({page})=>{
  await connect(page);const panel=page.locator('.project-panel');
  await page.getByLabel('Project file',{exact:true}).setInputFiles({name:'invalid.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify({version:1,name:'bad',token:'VERY-SECRET'}))});
  await expect(panel.getByRole('alert')).toContainText('Rust contract');await expect(panel.getByRole('alert')).not.toContainText('VERY-SECRET');
  const project={version:1,name:'bad',recipes:[{name:'secret',steps:[{op:'AES Decrypt',args:['VERY-SECRET'],enabled:false,breakpoint:false}]}]};
  await page.getByLabel('Project file',{exact:true}).setInputFiles({name:'secret.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify(project))});
  await expect(panel.getByRole('alert')).toContainText('sensitive');await expect(panel.getByRole('alert')).not.toContainText('VERY-SECRET');
  await expect(page.getByLabel('Project name',{exact:true})).toHaveValue('Workspace');
});

test('an input file finishing after a graph switch cannot bind private bytes into the new graph',async({page})=>{
  await connect(page);await page.getByRole('button',{name:'Graph mode',exact:true}).click();
  let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve});let fetched!:()=>void;const ready=new Promise<void>(resolve=>{fetched=resolve});
  await page.route('**/api/v1/capabilities',async route=>{const response=await route.fetch();fetched();await gate;await route.fulfill({response});});
  await page.getByLabel('Graph input file', {exact:true}).setInputFiles({name:'private.bin',mimeType:'application/octet-stream',buffer:Buffer.from('PRIVATE-OLD-GRAPH-INPUT')});
  await ready;await page.getByRole('button',{name:'New project graph',exact:true}).click();
  await expect(page.getByLabel('Graph name',{exact:true})).not.toHaveValue('untitled');
  await page.getByLabel('Graph input data').fill('new graph data');release();
  await expect(page.getByRole('alert')).toContainText('File import discarded');
  await expect(page.getByLabel('Graph input data')).toHaveValue('new graph data');
});
