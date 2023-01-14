Object.assign(Number.prototype, {
    $clone() {
        return new this.constructor(this.valueOf())
    },

    add(num) {
        return new this.constructor(this.valueOf() + num.valueOf())
    },

    subtract(num) {
        return new this.constructor(this.valueOf() - num.valueOf())
    },

    multiply(num) {
        return new this.constructor(this.valueOf() * num.valueOf())
    },

    divide(num) {
        const result = this.valueOf() / num.valueOf()
        const truncated = Math.trunc(result)
        return new this.constructor(truncated)
    },

    modulo(num) {
        return new this.constructor(this.valueOf() % num.valueOf())
    },

    power(num) {
        return new this.constructor(this.valueOf() ** num.valueOf())
    },

    equals(num) {
        return this.valueOf() === num.valueOf()
    },

    notEquals(num) {
        return this.valueOf() !== num.valueOf()
    },

    lessThan(num) {
        return this.valueOf() < num.valueOf()
    },

    lessThanOrEquals(num) {
        return this.valueOf() <= num.valueOf()
    },

    greaterThan(num) {
        return this.valueOf() > num.valueOf()
    },

    greaterThanOrEquals(num) {
        return this.valueOf() >= num.valueOf()
    },
})
