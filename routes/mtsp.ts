import { z } from "jsr:@zod/zod@4.1.12";
import { mtsp } from "../lib/mtsp.wasm";

// 스키마 정의
// [ { a: number, b: number, ... }, ... ]
const schema = z.array(z.object({
    lat: z.number(),
    lng: z.number(),
}).loose()).min(1).max(100);

const compute = (data: { lat: number, lng: number, [_: string]: unknown }[]) => {
    return data;
};

export const mtsp = {
    schema, compute
};
