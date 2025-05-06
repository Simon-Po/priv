
import { createHash } from "node:crypto"

export function sha256(content: String) {
  // simulate big work
  for (let i = 0; i < 8_000_000_000; i++) { }
  return createHash("sha256").update(Buffer.from(content, "utf8")).digest("hex")
}
