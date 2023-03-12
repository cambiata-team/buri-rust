import {
    BemptyListSize,
    BzeroThroughFiveSize,
    BzeroThroughFourSize,
} from "@tests/js/valid/list/size.mjs"
import { expect, it } from "bun:test"

it("[0, 1, 2, 3, 4]:size() == 0", () => {
    expect(BemptyListSize).toEqual(0)
})

it("[0, 1, 2, 3, 4]:size() == 5", () => {
    expect(BzeroThroughFourSize).toEqual(5)
})

it("[0, 1, 2, 3, 4, 5]:size() == 6", () => {
    expect(BzeroThroughFiveSize).toEqual(6)
})
