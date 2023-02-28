import {
    Bage,
    BretrievedAge,
} from "@tests/js/valid/functions/record-polymorphism.mjs"
import { expect, it } from "bun:test"

it("expect the retrieved age to equal the age", () => {
    expect(BretrievedAge).toEqual(Bage)
})
