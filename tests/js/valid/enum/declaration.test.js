import {
    BbooleanFalse,
    BbooleanTrue,
    Bgreen,
    Bhello,
    Blocalhost,
    Bred,
} from "@tests/js/valid/enum/declaration.mjs"
import { expect, it } from "bun:test"
import { getEnumContents, getEnumValue } from "../helpers"

it("compiler boolean true", () => {
    expect(BbooleanTrue).toBeTruthy()
})

it("compiler boolean false", () => {
    expect(BbooleanFalse).toBeFalsy()
})

it("enums are serialized to integers", () => {
    expect(typeof getEnumValue(Bhello)).toBe("number")
})

it("enums can have a payload", () => {
    expect(getEnumContents(Blocalhost)).toEqual([127, 0, 0, 1])
})

it("enum integer values are unique", () => {
    expect(getEnumValue(Bred)).not.toEqual(getEnumValue(Bgreen))
})
