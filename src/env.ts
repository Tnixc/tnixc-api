/* eslint-disable node/no-process-env */
import { z } from 'zod';

const EnvSchema = z.object({
  NODE_ENV: z
    .enum(['development', 'production', 'test'])
    .default('development'),
  PORT: z.coerce.number().default(3000),
  DATABASE_URL: z.string().url(),
  RUNTIME: z.enum(['bun', 'edge']).default('bun'),
});

export type env = z.infer<typeof EnvSchema>;

// eslint-disable-next-line ts/no-redeclare
const { data: env, error } = EnvSchema.safeParse(process.env);

// if (error) {
//   console.error('❌ Invalid env:');
//   console.error(JSON.stringify(error.flatten().fieldErrors, null, 2));
//   process.exit(1);
// }

export default env!;
