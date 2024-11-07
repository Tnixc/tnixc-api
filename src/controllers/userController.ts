import { okResponse } from '@/utils/response';
import { Context } from 'elysia';

export async function getAllUser(c: Context) {
  const users = [
    {
      id: 1,
      name: 'John Doe',
    },
    {
      id: 2,
      name: 'Jane Doe',
    },
    {
      id: 3,
      name: 'Foo Bar',
    },
  ];

  return okResponse(users);
}
