import { BthreeElseFive } from "@tests/js/valid/declarations/indented.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("threeElseFive", () => {
    it("should return 3 when #true", () => {
        expect(BthreeElseFive(tag("true")).valueOf()).toEqual(3)
    })

    it("should return 5 when #false", () => {
        expect(BthreeElseFive(tag("false")).valueOf()).toEqual(5)
    })
})
