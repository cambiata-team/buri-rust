import {
    hasArray,
    nested,
    person,
} from "@tests/js/valid/record/definitions.mjs"

import { expect, it } from "bun:test"

it("records can contain fields of different types", () => {
    expect(person).toEqual({
        name: "Jane Doe",
        age: 30,
    })
})

it("records can be nested", () => {
    expect(nested).toEqual({
        name: "Mary Doe",
        mom: {
            name: "Jane Doe",
            age: 30,
        },
        dad: {
            name: "John Doe",
            age: 30,
        },
    })
})

it("records can contain arrays", () => {
    expect(hasArray).toEqual({ a: [1, 2, 3] })
})
