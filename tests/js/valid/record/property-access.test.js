import {
    Bperson,
    Bperson_name,
    Bperson_age,
    Bperson_eye_color,
    Bnest,
    Bnest_bird_egg_id,
} from "@tests/js/valid/record/property-access.mjs"
import { expect, it } from "bun:test"

it("person.name can be accessed", () => {
    expect(Bperson_name).toEqual(Bperson.name)
})

it("person.age can be accessed", () => {
    expect(Bperson_age).toEqual(Bperson.age)
})

it("person.eye_color can be accessed", () => {
    expect(Bperson_eye_color).toEqual(Bperson.eye_color)
})

it("nested properties can be accessed", () => {
    expect(Bnest_bird_egg_id).toEqual(Bnest.bird.egg.id)
})
