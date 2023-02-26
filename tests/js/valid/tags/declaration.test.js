import {
    BbooleanFalse,
    BbooleanTrue,
    Bhello,
    BlocalHost,
} from "@tests/js/valid/tags/declaration.mjs"
import { expect, it } from "bun:test"
import { getTagContents, getTagName } from "../helpers"

it("compiler boolean true", () => {
    expect(BbooleanTrue).toBe(true)
})

it("compiler boolean false", () => {
    expect(BbooleanFalse).toBe(false)
})

it("tags can have a name", () => {
    expect(getTagName(Bhello)).toBe("hello")
})

it("tags can hold contents", () => {
    expect(getTagContents(BlocalHost)).toEqual([127, 0, 0, 1])
})
