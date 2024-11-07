import { getAppDetail } from '../controllers/homeController';
import { getAllUser } from '../controllers/userController';
import Elysia from 'elysia';

const appRoute = new Elysia();

appRoute.get('/', getAppDetail);
appRoute.get('/users', getAllUser);

export default appRoute;
