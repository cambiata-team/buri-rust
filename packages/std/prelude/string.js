Object.assign(String.prototype, {
    $clone() {
        return new this.constructor(this.valueOf())
    },
})
