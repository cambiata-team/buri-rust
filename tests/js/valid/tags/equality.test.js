import {
    BgreenEqualsBlue,
    BgreenEqualsGreen,
    BgreenNotEqualsBlue,
    BgreenNotEqualsGreen,
} from "@tests/js/valid/tags/equality.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("#green == #green -- #true", () => {
    expect(BgreenEqualsGreen).toEqual(tag("true"))
})

it("#green == #blue -- #false", () => {
    expect(BgreenEqualsBlue).toEqual(tag("false"))
})

it("#green != #green -- #false", () => {
    expect(BgreenNotEqualsGreen).toEqual(tag("false"))
})

it("#green != #blue -- #true", () => {
    expect(BgreenNotEqualsBlue).toEqual(tag("true"))
})
