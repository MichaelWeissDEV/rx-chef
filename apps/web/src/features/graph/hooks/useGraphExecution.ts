import { useEffect, useState } from 'react';
import type { RxchefClient } from '../../../types';
import { GraphExecution, type GraphRunState } from '../GraphExecution';
export function useGraphExecution(client: RxchefClient) {
  const [controller] = useState(() => new GraphExecution());
  const [record, setRecord] = useState<{client: RxchefClient; state: GraphRunState}>({ client, state: controller.state });
  useEffect(() => {
    const unsubscribe = controller.subscribe(state => setRecord({ client, state })); controller.connect(client);
    return () => { unsubscribe(); controller.close(); };
  }, [controller, client]);
  return { state: record.client === client ? record.state : { status: 'idle' as const, result: null, canCancel: false }, controller };
}
