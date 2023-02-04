import {
    listOfNumbers,
    listOfStrings,
    nestedLists,
} from "@tests/js/valid/list/definitions.mjs"

import { expect, it } from "bun:test"

it("a list can contain numbers", () => {
    expect(listOfNumbers).toEqual([0, 1, 2, 3])
})

it("a list can contain strings", () => {
    expect(listOfStrings).toEqual(["a", "b", "c"])
})

it("a list can be nested", () => {
    expect(nestedLists).toEqual([
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
    ])
})
