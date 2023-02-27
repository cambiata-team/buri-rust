import {
    BtrueAndFalse,
    BfalseAndTrue,
    BtrueOrFalse,
    BfalseOrTrue,
} from "@tests/js/valid/logical-operators/definitions.mjs"
import { expect, it } from "bun:test"

it("true and false == false and true", () => {
    expect(BtrueAndFalse).toEqual(BfalseAndTrue)
})

it("true or false == false or true", () => {
    expect(BtrueOrFalse).toEqual(BfalseOrTrue)
})
