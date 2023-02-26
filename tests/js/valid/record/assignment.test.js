import {
    BjohnSkipsAYear,
    BoriginalTeacher,
    BsubstituteTeacher,
    BsubstituteTeacherName,
} from "@tests/js/valid/record/assignment.mjs"
import { expect, it } from "bun:test"

it('the original name is "Mr. Knowitall"', () => {
    expect(BoriginalTeacher.name).toBe("Mr. Knowitall")
})

it('the substitute name should be "Mr. Fillin"', () => {
    expect(BsubstituteTeacher.name).toBe(BsubstituteTeacherName)
})

it("can have nested record updates", () => {
    expect(BoriginalTeacher.students.john.grade).toBe(11)
    expect(BjohnSkipsAYear.students.john.grade).toBe(12)
})
