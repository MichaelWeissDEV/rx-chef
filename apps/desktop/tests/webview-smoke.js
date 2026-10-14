(async () => {
  if (window.__rxchefSmokeStarted) return;
  window.__rxchefSmokeStarted = true;
  const invoke = window.__TAURI_INTERNALS__.invoke;
  const wait = predicate => new Promise((resolve, reject) => {
    if (predicate()) { resolve(); return; }
    // DOM updates also arrive when WebKit throttles background-window timers.
    const observer = new MutationObserver(() => { if (predicate()) { clearTimeout(timeout); observer.disconnect(); resolve(); } });
    const timeout = setTimeout(() => { observer.disconnect(); reject(new Error('UI timeout for ' + predicate.toString() + ': ' + document.body.textContent.slice(-1600))); }, 10000);
    observer.observe(document.body, {childList:true, subtree:true, characterData:true});
  });
  try {
    await invoke('ui_test_progress', {stage:'waiting for registry'});
    await wait(() => document.body.textContent.includes('Operations (478)'));
    const bake = [...document.querySelectorAll('button')].find(button => button.textContent.trim() === 'Bake');
    if (!bake) throw new Error('Bake action missing');
    bake.click();
    await invoke('ui_test_progress', {stage:'waiting for recipe output'});
    await wait(() => /Output\s*\(40 bytes[,)]/.test(document.body.textContent));
    const data = btoa(String.fromCharCode(...Array.from({ length: 256 }, (_, i) => i)));
    await invoke('ui_test_progress', {stage:'binary worker execution'});
    const response = await invoke('engine_call', { method: 'execute', params: { input_base64: data, recipe: [] } });
    if (response.result.output_base64 !== data || response.result.output_is_utf8) throw new Error('Binary IPC mismatch');
    const graph = { version: 1, id: 'native-smoke', nodes: [
      { id: 'input', type: 'input', name: 'input' }, { id: 'adapter', type: 'argument', kind: 'bytes' },
      { id: 'recipe', type: 'recipe', steps: [{op:'to_base64', args:[]}, {op:'from_base64', args:[]}] }, {id:'output',type:'output',name:'output'}],
      edges: [['input','adapter'],['adapter','recipe'],['recipe','output']].map(([source_node,target_node],i) => ({id:`edge-${i}`,source_node,target_node,source_port:'data',target_port:'data'})) };
    await invoke('ui_test_progress', {stage:'graph worker execution'});
    const graphResult = await invoke('engine_call', {method:'graph_execute', params:{graph, inputs_base64:{input:data}, outputs:[]}});
    if (graphResult.outputs[0].output_base64 !== data) throw new Error('Native graph bytes mismatch');
    const project = await invoke('engine_call', {method:'project_validate',params:{version:1,name:'Native project',graphs:[graph]}});
    if (project.graphs[0].id !== graph.id || JSON.stringify(project).includes(data)) throw new Error('Native project contract mismatch');
    const slow = {method:'execute',params:{input_base64:'',recipe:[{op:'Derive PBKDF2 key',args:['password','128','4000000000','SHA256','salt']}]}};
    const job = await invoke('engine_call', {method:'submit_job',params:slow});
    await invoke('ui_test_progress', {stage:'cancellation'});
    await new Promise(resolve => setTimeout(resolve, 200));
    const cancelled = await invoke('engine_call', {method:'cancel_job', params:{id:job.id}});
    if (cancelled.status !== 'cancelled' || cancelled.result) throw new Error('Native cancellation did not terminate work');
    await invoke('configure_connection', { url: null, token: null });
    await invoke('ui_test_result', { result: { passed: true, operations: 478, bytes: 256, graph: true, project:true, cancellation: true, ui: document.body.innerText.slice(0, 800) } });
  } catch (error) { await invoke('ui_test_result', { result: { passed: false, error: String(error) } }); }
})();
