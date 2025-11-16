import { Hono } from "jsr:@hono/hono@4.10.5";
import { cors } from "jsr:@hono/hono@4.10.5/cors"
import { zValidator } from "jsr:@hono/zod-validator@0.7.4";
import { test } from "./routes/test.ts";
import { mtsp } from "./routes/mtsp.ts";


const app = new Hono();

app.use("*", cors({
  origin: [
    "http://localhost:4321",
    "https://ehzilehzil.github.io"
  ],
  allowMethods: ["GET", "POST", "OPTIONS"],
  allowHeaders: ["Content-Type", "Authorization"],
}));

app.post("/test", zValidator("json", test.schema), async (c) => {
  const data = c.req.valid("json");
  return c.json(test.sum(data));
});

app.post("/mtsp", zValidator("json", mtsp.schema), async (c) => {
  const data = c.req.valid("json");
  return c.json(JSON.parse(mtsp.compute(data)));
});

Deno.serve(/*{ port: 8000 }, */app.fetch);