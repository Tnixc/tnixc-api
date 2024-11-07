import env from '@/env';
import { okResponse } from '@/utils/response';
import { Context } from 'elysia';

export async function getAppDetail(c: Context) {
  return okResponse({
    name: 'Elysia',
    version: '1.0.0',
    mode: env.NODE_ENV || 'development',
    description: 'A web framework for Deno',
  });
}
