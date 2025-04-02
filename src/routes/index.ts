import Elysia from 'elysia';

const appRoute = new Elysia();

appRoute.get('/', () => "hello elysia~");

export default appRoute;
