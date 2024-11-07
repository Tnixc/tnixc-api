import { Elysia } from 'elysia';
import dotenv from 'dotenv';
import env from './env';
import appRoute from './routes';

dotenv.config();

const app = new Elysia().use(appRoute);

const port = env.PORT || 3000;

app.listen(port);

export default app;
