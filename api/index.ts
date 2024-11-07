// eslint-disable-next-line ts/ban-ts-comment
// @ts-expect-error
// eslint-disable-next-line antfu/no-import-dist
import { app } from '../dist/src/app.js';

// export const config = { runtime: 'edge' };
export const runtime = 'edge';

export const GET = async (request: Request) => {
  return app.fetch(request);
};

export const POST = async (request: Request) => {
  return app.fetch(request);
};

export const PUT = async (request: Request) => {
  return app.fetch(request);
};

export const DELETE = async (request: Request) => {
  return app.fetch(request);
};

export const PATCH = async (request: Request) => {
  return app.fetch(request);
};
