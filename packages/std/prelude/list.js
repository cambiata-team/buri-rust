Object.assign(Array.prototype, {
    $clone() {
        return new this.constructor(...this.valueOf())
    },

    equals(other) {
        return this[0] == other[0]
    },

    notEquals(other) {
        return this[0] != other[0]
    },

    get(index) {
        if (index > this.length || index < 0) {
            return ["none"]
        }
        return ["some", this[index]]
    },

    append(value) {
        return [...this, value]
    },
})
