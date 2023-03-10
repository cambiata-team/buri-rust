import {
    BzeroThroughFive,
    BzeroThroughFour,
} from "@tests/js/valid/list/append.mjs"
import { expect, it } from "bun:test"

it("[0, 1, 2, 3, 4]", () => {
    expect(BzeroThroughFour).toEqual([0, 1, 2, 3, 4])
})

it("[0, 1, 2, 3, 4]:append(5)", () => {
    expect(BzeroThroughFive).toEqual([0, 1, 2, 3, 4, 5])
})
