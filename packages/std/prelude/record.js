Object.assign(Object.prototype, {
    $clone() {
        return new this.constructor({ ...this.valueOf() })
    },

    /** Allows us to set a key on an object without mutating the original object.
     *
     * In Buri, you can write the following code:
     *
     * ```buri
     * person = { name: "Sam", age: 30 }
     * theodore = { person | name: "Theodore" }
     * ```
     *
     * This is compiled to the following JS code:
     *
     * ```js
     * let person = { name: "Sam", age: 30 }
     * let theodore = person.$set({ name: "Theodore" })
     * ```
     *
     * It's worth noting that in Buri you can assign multiple keys at once:
     *
     * ```buri
     * person = { name: "Sam", age: 30 }
     * theodore = { person | name: "Theodore", age: 31 }
     * ```
     *
     * This is compiled to the following JS code:
     *
     * ```js
     * let person = { name: "Sam", age: 30 }
     * let theodore = person.$set({ name: "Theodore", age: 31 })
     * ```
     */
    $set(newValues) {
        return new this.constructor({ ...this.valueOf(), ...newValues })
    },
})
