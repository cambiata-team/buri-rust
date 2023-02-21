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
        let result = this.valueOf() * num.valueOf()
        if (result === -0) result = 0
        return new this.constructor(result)
    },

    divide(num) {
        const result = this.valueOf() / num.valueOf()
        const truncated = Math.trunc(result)
        if (truncated === -0) return new this.constructor(0)
        return new this.constructor(truncated)
    },

    // Using a custom modulo function because the built-in one is mathematically incorrect.
    // https://stackoverflow.com/questions/4467539/javascript-modulo-gives-a-negative-result-for-negative-numbers
    modulo(num) {
        let m = this.valueOf()
        let n = num.valueOf()
        let result = ((m % n) + n) % n
        if (result === -0) result = 0
        return new this.constructor(result)
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
