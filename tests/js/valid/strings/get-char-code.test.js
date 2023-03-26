import { BcodeOfLowercaseH, BcodeOfLowercaseE, BcodeOfLowercaseO, BcodeOfLowercaseL, BcodeOfIndexBefore, BcodeOfIndexAfter } from "@tests/js/valid/strings/get-char-code.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("\"hello\":getCharCode(0) == #some(104)", () => {
    expect(BcodeOfLowercaseH).toEqual(tag("some", 104))
})

it("\"hello\":getCharCode(1) == #some(101)", () => {
    expect(BcodeOfLowercaseE).toEqual(tag("some", 101))
})

it("\"hello\":getCharCode(-1) == #some(111)", () => {
    expect(BcodeOfLowercaseO).toEqual(tag("some", 111))
})

it("\"hello\":getCharCode(-2) == #some(108)", () => {
    expect(BcodeOfLowercaseL).toEqual(tag("some", 108))
})

it("\"hello\":getCharCode(-1) == #none", () => {
    expect(BcodeOfIndexBefore).toEqual(tag("none"))
})

it("\"hello\":getCharCode(50) == #none", () => {
    expect(BcodeOfIndexAfter).toEqual(tag("none"))
})
