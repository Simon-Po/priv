import { Worker, isMainThread } from "node:worker_threads"
import * as Bun from "bun"




Bun.serve(
  {
    port: 3030,
    routes: {
      "/": new Response("Hey there"),
      "/sha": async (req: Bun.BunRequest<"/sha">) => {
        const worker = new Worker("worker.ts")
        worker.on("message", (msg) => {
          return new Response(msg)
        }
        )
        
      },
    }
  }
)





