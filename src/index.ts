import app from './app';

console.log(
  `🦊 Elysia is running at http://${app.server?.hostname}:${app.server?.port}`
);

Bun.serve({
  fetch: app.fetch,
  port: app.server?.port,
});
