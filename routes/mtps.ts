import { z } from "zod";

// 스키마 정의
const schema = z.array(z.object({
    lng: z.number(),
    lat: z.number(),
}).loose()).min(1).max(100);



const mtsp = {

};

export default mtsp;
