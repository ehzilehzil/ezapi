import { z } from "jsr:@zod/zod@4.1.12";
// import { get_mtsp } from "../lib/mtsp.wasm";
import { get_mtsp } from "../lib/mtsp.js";

// 스키마 정의
// [ { a: number, b: number, ... }, ... ], k: number
const schema = z.object({
    items: z.array(z.object({
        name: z.string(),
        addr: z.string(),
        lat: z.number(),
        lng: z.number(),
        g: z.number(),
    })).min(1).max(100),
    k: z.number().min(1).max(10),
});

type Item = {
    name: string,
    addr: string,
    lat: number,
    lng: number,
    g: number,
};

const compute = (data: {items: Item[], k: number}) => {
    let {items, k} = data;
    return get_mtsp(JSON.stringify(items), k);
};

export const mtsp = {
    schema, compute
};
