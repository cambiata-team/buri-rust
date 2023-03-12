Object.assign(String.prototype, {
    $clone() {
        return new this.constructor(this.valueOf())
    },

    size() {
        return this.length
    },
})
