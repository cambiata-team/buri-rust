import { expect, it } from "bun:test"
import "./index.js"

it("cloning true should still be true", () => {
    const bool = true
    expect(bool.$clone()).toBe(true)
})

it("cloning false should still be false", () => {
    const bool = false
    expect(bool.$clone()).toBe(false)
})
