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

    size() {
        return this.length
    },

    mapWithResult(func) {
        let results = []
        for (let i = 0; i < this.length; i++) {
            let result = func(this[i])
            if (result[0] === "error") {
                return result
            }
            else {
                results.push(result[1])
            }
        }
        return ["ok", results]
    }
})
