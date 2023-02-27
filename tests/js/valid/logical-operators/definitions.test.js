import {
    BfalseAndFalse,
    BfalseAndTrue,
    BfalseOrFalse,
    BfalseOrTrue,
    BnotFalse,
    BnotTrue,
    BtrueAndFalse,
    BtrueAndTrue,
    BtrueOrFalse,
    BtrueOrTrue,
} from "@tests/js/valid/logical-operators/definitions.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("and", () => {
    it("#true and #true -- #true", () => {
        expect(BtrueAndTrue).toEqual(tag("true"))
    })

    it("#true and #false -- #false", () => {
        expect(BtrueAndFalse).toEqual(tag("false"))
    })

    it("#false and #true -- #false", () => {
        expect(BfalseAndTrue).toEqual(tag("false"))
    })

    it("#false and #false -- #false", () => {
        expect(BfalseAndFalse).toEqual(tag("false"))
    })
})

describe("or", () => {
    it("#true or #true -- #true", () => {
        expect(BtrueOrTrue).toEqual(tag("true"))
    })

    it("#true or #false -- #true", () => {
        expect(BtrueOrFalse).toEqual(tag("true"))
    })

    it("#false or #true -- #true", () => {
        expect(BfalseOrTrue).toEqual(tag("true"))
    })

    it("#false or #false -- #false", () => {
        expect(BfalseOrFalse).toEqual(tag("false"))
    })
})

describe("not", () => {
    it("not #true -- #false", () => {
        expect(BnotTrue).toEqual(tag("false"))
    })

    it("not #false -- #true", () => {
        expect(BnotFalse).toEqual(tag("true"))
    })
})
