import { threeElseFive } from "@tests/js/valid/declarations/indented.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("threeElseFive", () => {
    it("should return 3 when #true", () => {
        expect(threeElseFive(tag("true")).valueOf()).toEqual(3)
    })

    it("should return 5 when #false", () => {
        expect(threeElseFive(tag("false")).valueOf()).toEqual(5)
    })
})
