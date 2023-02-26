import { getTagContents, getTagName } from "@tests/js/valid/helpers"
import {
    BmultilineNone,
    BmultilineSomeFive,
    Bnone,
    BsomeFive,
} from "@tests/js/valid/if/no-else.mjs"
import { describe, expect, it } from "bun:test"

describe("single line if expressions", () => {
    it("if without else returns #some when condition is true", () => {
        expect(getTagName(BsomeFive)).toBe("some")
    })

    it("if without else returns tag whose contents are the value of the expression", () => {
        expect(getTagContents(BsomeFive)).toEqual([5])
    })

    it("if without else returns #none when condition is false", () => {
        expect(getTagName(Bnone)).toBe("none")
    })
})

describe("multiline if expressions", () => {
    it("if without else returns #some when condition is true", () => {
        expect(getTagName(BmultilineSomeFive)).toBe("some")
    })

    it("if without else returns tag whose contents are the value of the expression", () => {
        expect(getTagContents(BmultilineSomeFive)).toEqual([5])
    })

    it("if without else returns #none when condition is false", () => {
        expect(getTagName(BmultilineNone)).toBe("none")
    })
})
