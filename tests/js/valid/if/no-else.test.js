import { getTagContents, getTagName } from "@tests/js/valid/helpers"
import {
    multilineNone,
    multilineSomeFive,
    none,
    someFive,
} from "@tests/js/valid/if/no-else.mjs"
import { describe, expect, it } from "bun:test"

describe("single line if expressions", () => {
    it("if without else returns #some when condition is true", () => {
        expect(getTagName(someFive)).toBe("some")
    })

    it("if without else returns tag whose contents are the value of the expression", () => {
        expect(getTagContents(someFive)).toEqual([5])
    })

    it("if without else returns #none when condition is false", () => {
        expect(getTagName(none)).toBe("none")
    })
})

describe("multiline if expressions", () => {
    it("if without else returns #some when condition is true", () => {
        expect(getTagName(multilineSomeFive)).toBe("some")
    })

    it("if without else returns tag whose contents are the value of the expression", () => {
        expect(getTagContents(multilineSomeFive)).toEqual([5])
    })

    it("if without else returns #none when condition is false", () => {
        expect(getTagName(multilineNone)).toBe("none")
    })
})
