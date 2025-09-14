import * as z from "zod";

export const env = z
  .object({
    isProd: z.boolean()
  })
  .parse({
    isProd: import.meta.env.NODE_ENV === "production"
  });
