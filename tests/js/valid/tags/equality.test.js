import {
    greenEqualsBlue,
    greenEqualsGreen,
    greenNotEqualsBlue,
    greenNotEqualsGreen,
} from "@tests/js/valid/tags/equality.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("#green == #green -- #true", () => {
    expect(greenEqualsGreen).toEqual(tag("true"))
})

it("#green == #blue -- #false", () => {
    expect(greenEqualsBlue).toEqual(tag("false"))
})

it("#green != #green -- #false", () => {
    expect(greenNotEqualsGreen).toEqual(tag("false"))
})

it("#green != #blue -- #true", () => {
    expect(greenNotEqualsBlue).toEqual(tag("true"))
})
