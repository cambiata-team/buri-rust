import {
    originalTeacher,
    substituteTeacher,
} from "@tests/js/valid/record/assignment.mjs"
import { expect, it } from "bun:test"

it('the original name is "Mr. Knowitall"', () => {
    expect(originalTeacher.name).toBe("Mr. Knowitall")
})

it('the substitute name should be "Mr. Fillin"', () => {
    expect(substituteTeacher.name).toBe("Mr. Fillin")
})
