import { createElysia } from '@/utils/elysia';
import { okResponse } from './utils/response';
import appRoute from './routes';

export const app = createElysia()
  .use(appRoute)
  .get('/health', () => okResponse({ status: 'ok' }));

export type App = typeof app;
