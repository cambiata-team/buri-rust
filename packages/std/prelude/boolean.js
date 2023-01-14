Object.assign(Boolean.prototype, {
    $clone() {
        return this.valueOf()
    },
})
