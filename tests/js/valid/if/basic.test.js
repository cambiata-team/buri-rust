import {
    Bfive,
    BmultilineFive,
    BmultilineThree,
    Bthree,
} from "@tests/js/valid/if/basic.mjs"
import { expect, it } from "bun:test"

it("should evaluate the true branch", () => {
    expect(Bfive).toBe(5)
})

it("should evaluate the false branch", () => {
    expect(Bthree).toBe(3)
})

it("should evaluate the true branch of a multiline if", () => {
    expect(BmultilineFive).toBe(5)
})

it("should evaluate the false branch of a multiline if", () => {
    expect(BmultilineThree).toBe(3)
})
