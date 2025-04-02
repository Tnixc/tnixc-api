
import {app} from '../src/app'

export const config = { runtime: 'edge' };

const handler = async (request: Request) => {
  return app.fetch(request);
};

export const GET = handler;
export const POST = handler;
export const PUT = handler;
export const DELETE = handler;
export const PATCH = handler;
