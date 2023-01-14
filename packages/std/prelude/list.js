Object.assign(Array.prototype, {
    $clone() {
        return new this.constructor(...this.valueOf())
    },
})
