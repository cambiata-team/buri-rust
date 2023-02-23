import {
    booleanFalse,
    booleanTrue,
    hello,
    localHost,
} from "@tests/js/valid/tags/declaration.mjs"
import { expect, it } from "bun:test"
import { getTagContents, getTagName } from "../helpers"

it("compiler boolean true", () => {
    expect(booleanTrue).toBe(true)
})

it("compiler boolean false", () => {
    expect(booleanFalse).toBe(false)
})

it("tags can have a name", () => {
    expect(getTagName(hello)).toBe("hello")
})

it("tags can hold contents", () => {
    expect(getTagContents(localHost)).toEqual([127, 0, 0, 1])
})
