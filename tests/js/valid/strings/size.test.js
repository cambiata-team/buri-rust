import { BhelloSize, BhelloWorldSize } from "@tests/js/valid/strings/size.mjs"

import { expect, it } from "bun:test"

it('"hello":size() == 5', () => {
    expect(BhelloSize.valueOf()).toBe(5)
})

it('"hello world":size() == 11', () => {
    expect(BhelloWorldSize.valueOf()).toBe(11)
})
