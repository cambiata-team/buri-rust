import {
    johnSkipsAYear,
    originalTeacher,
    substituteTeacher,
    substituteTeacherName,
} from "@tests/js/valid/record/assignment.mjs"
import { expect, it } from "bun:test"

it('the original name is "Mr. Knowitall"', () => {
    expect(originalTeacher.name).toBe("Mr. Knowitall")
})

it('the substitute name should be "Mr. Fillin"', () => {
    expect(substituteTeacher.name).toBe(substituteTeacherName)
})

it("can have nested record updates", () => {
    expect(originalTeacher.students.john.grade).toBe(11)
    expect(johnSkipsAYear.students.john.grade).toBe(12)
})
