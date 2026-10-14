import { describe, expect, it } from 'vitest';
import type { Graph } from '../../types';
import { arrangeGraph } from './layout';
describe('graph layout', () => {
  const graph: Graph = { version: 1, id: 'layout', nodes: [
    {id:'in', type:'input', name:'input'}, {id:'a', type:'operation', operation:'to_base64'},
    {id:'b', type:'operation', operation:'to_hex'}, {id:'out', type:'output', name:'output'},
    {id:'isolated', type:'input', name:'other'}], edges: [
      ['in','a'], ['in','b'], ['a','out'], ['b','out']
    ].map(([source_node,target_node], i) => ({id:`e${i}`, source_node,target_node,source_port:'data',target_port:'data'})) };
  it('puts fan-out branches in non-overlapping layers using measured dimensions', () => {
    const positions = arrangeGraph(graph, {a:{width:500,height:400}});
    expect(positions.in.x).toBeLessThan(positions.a.x);
    expect(positions.a.x).toBe(positions.b.x);
    expect(positions.out.x).toBeGreaterThanOrEqual(positions.a.x + 600);
    expect(positions.b.y).toBeGreaterThanOrEqual(positions.a.y + 460);
    expect(Object.keys(positions)).toHaveLength(graph.nodes.length);
    expect(arrangeGraph(graph)).toEqual(arrangeGraph(graph));
  });
  it('keeps cyclic and disconnected drafts finite without altering their definitions', () => {
    const draft = structuredClone(graph); draft.edges.push({id:'cycle',source_node:'a',target_node:'in',source_port:'data',target_port:'data'});
    const before = structuredClone(draft); const positions = arrangeGraph(draft);
    expect(Object.values(positions).every(p => Number.isFinite(p.x) && Number.isFinite(p.y))).toBe(true);
    expect(draft).toEqual(before);
  });
});
