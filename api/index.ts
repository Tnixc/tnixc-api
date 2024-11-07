// eslint-disable-next-line ts/ban-ts-comment
// @ts-expect-error
// eslint-disable-next-line antfu/no-import-dist
import { app } from '../dist/src/app';

// export const config = { runtime: 'edge' };
export const runtime = 'edge';

export default async function handler(request: Request) {
  return app.fetch(request);
}
