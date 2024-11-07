// eslint-disable-next-line ts/ban-ts-comment
// eslint-disable-next-line antfu/no-import-dist
import '../src/env';

import { app } from '../src/app';

export const config = { runtime: 'edge' };

export default async function handler(request: Request) {
  return app.fetch(request);
}
