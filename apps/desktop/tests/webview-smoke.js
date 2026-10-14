(async () => {
  if (window.__rxchefSmokeStarted) return;
  window.__rxchefSmokeStarted = true;
  const invoke = window.__TAURI_INTERNALS__.invoke;
  const wait = async predicate => { for (let i = 0; i < 100; i++) { if (predicate()) return; await new Promise(resolve => setTimeout(resolve, 100)); } throw new Error('UI timeout: ' + document.body.innerText.slice(0, 600)); };
  try {
    await wait(() => document.body.innerText.includes('Operations (478)'));
    const bake = [...document.querySelectorAll('button')].find(button => button.textContent.trim() === 'Bake');
    if (!bake) throw new Error('Bake action missing');
    bake.click();
    await wait(() => document.body.innerText.includes('Output (36 bytes)'));
    const data = btoa(String.fromCharCode(...Array.from({ length: 256 }, (_, i) => i)));
    const response = await invoke('engine_call', { method: 'execute', params: { input_base64: data, recipe: [] } });
    if (response.result.output_base64 !== data || response.result.output_is_utf8) throw new Error('Binary IPC mismatch');
    await invoke('configure_connection', { url: null, token: null });
    await invoke('ui_test_result', { result: { passed: true, operations: 478, bytes: 256, ui: document.body.innerText.slice(0, 800) } });
  } catch (error) { await invoke('ui_test_result', { result: { passed: false, error: String(error) } }); }
})();
