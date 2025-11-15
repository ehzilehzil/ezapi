import { Hono } from "jsr:@hono/hono@4.10.5";
import { ZodError } from "jsr:@zod/zod@4.1.12";
import { test } from "./routes/test.ts";
import { mtsp } from "./routes/mtsp.ts";


const app = new Hono();

app.onError((err, c) => {
  console.log(err.name);

  if (err instanceof ZodError) {
    return c.json({
      success: false,
      message: "Validation failed",
      errors: err.issues.map((x) => ({
        path: x.path.join(", "),
        message: x.message,
      })),
    }, 400);
  } else if (err instanceof SyntaxError) {
    return c.json({
      success: false,
      message: err.message,
    }, 400);
  } else {
    return c.json({
      success: false,
      message: err.message || "Internal server error",
    }, 500);
  }
});

app.post("/test", async (c) => {
  const data = await c.req.json();
  const parsed_data = test.schema.safeParse(data);
  if (!parsed_data.success) throw parsed_data.error;

  return c.json(test.sum(data));
});

app.post("/mtsp", async (c) => {
  const data = await c.req.json();
  const parsed_data = mtsp.schema.safeParse(data);
  if (!parsed_data.success) throw parsed_data.error;

  return c.json(JSON.parse(mtsp.compute(data)));
});

Deno.serve(/*{ port: 8000 }, */app.fetch);

// app.post("/mtsp", zValidator("json", mtsp.schema), async (c) => {
//   const data = c.req.valid("json");
//   return c.json(JSON.parse(mtsp.compute(data)));
// });