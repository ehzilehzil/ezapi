import { z } from "jsr:@zod/zod@4.1.12";
import { add } from "../lib/add.wasm";

// 스키마 정의
// [ { a: number, b: number, ... }, ... ]
const schema = z.array(z.object({
    a: z.number(),
    b: z.number(),
}).loose()).min(1).max(10);

const sum = (data: { a: number, b: number, [_: string]: unknown }[]) => {
    // return data.map((x) => x.a + x.b );
    return data.map((x) => add(x.a, x.b));
};

export const test = {
    schema, sum
};
