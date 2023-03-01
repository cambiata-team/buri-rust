import {
    BtrueAndTrue,
    BtrueAndFalse,
    BfalseAndTrue,
    BfalseAndFalse,
    BtrueOrTrue,
    BtrueOrFalse,
    BfalseOrTrue,
    BfalseOrFalse,
    BnotTrue,
    BnotFalse,
} from "@tests/js/valid/logical-operators/function-return.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("and", () => {
    it("true and true -- true", () => {
        expect(BtrueAndTrue).toEqual(true)
    })
    it("true and false -- false", () => {
        expect(BtrueAndFalse).toEqual(false)
    })
    it("false and true -- false", () => {
        expect(BfalseAndTrue).toEqual(false)
    })
    it("false and false -- false", () => {
        expect(BfalseAndFalse).toEqual(false)
    })
})

describe("or", () => {
    it("true or true -- true", () => {
        expect(BtrueOrTrue).toEqual(true)
    })
    it("true or false -- true", () => {
        expect(BtrueOrFalse).toEqual(true)
    })
    it("false or true -- true", () => {
        expect(BfalseOrTrue).toEqual(true)
    })
    it("false or false -- false", () => {
        expect(BfalseOrFalse).toEqual(false)
    })
})

describe("not", () => {
    it("not true -- false", () => {
        expect(BnotTrue).toEqual(false)
    })
    it("not false -- true", () => {
        expect(BnotFalse).toEqual(true)
    })
})
