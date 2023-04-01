import { Bhello, Blocalhost } from "@tests/js/valid/enum/declaration.mjs"
import { expect, it } from "bun:test"
import { getEnumContents, getEnumValue } from "../helpers"

it("enums are serialized to integers", () => {
    expect(typeof getEnumValue(Bhello)).toBe("number")
})

it("enums can have a payload", () => {
    expect(getEnumContents(Blocalhost)).toEqual([127, 0, 0, 1])
})
