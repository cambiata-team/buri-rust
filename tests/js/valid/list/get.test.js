import { Bnone, BsomeOne, BsomeThree } from "@tests/js/valid/list/get.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("[0, 1, 2, 3, 4]:get(3) == #some(3)", () => {
    expect(BsomeThree).toEqual(tag("some", 3))
})

it("[0, 1, 2, 3, 4]:get(50) == #none", () => {
    expect(Bnone).toEqual(tag("none"))
})

it("[0, 1, 2, 3, 4]:get(one()) == #some(1)", () => {
    expect(BsomeOne).toEqual(tag("some", 1))
})
