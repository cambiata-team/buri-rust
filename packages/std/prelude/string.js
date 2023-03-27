const isUtf16SurrogateAtIndex = (string, jsIndex) => (string.charCodeAt(jsIndex) & 0xF800) === 0xD800

/**
 * Assume that the string is valid UTF-16.
 * Return the offset of the nth character in the string.
 * If n is negative, return the offset of the character that is nth from the end.
 * If the nth character is composed of a surrogate pair, return the offset of the high surrogate.
 * If n is out of bounds, return -1.
 */
const jsIndexForNthCharacter = (string, n) => {
    let step = n >= 0 ? 1 : -1
    let characterIndex = n >= 0 ? 0 : -1
    let jsIndex = n >= 0 ? 0 : string.length - 1

    while (jsIndex >= 0 && jsIndex < string.length) {
        if (characterIndex === n) {
            if (step === -1 && isUtf16SurrogateAtIndex(string, jsIndex)) {
                return jsIndex - 1
            }
            return jsIndex
        } else {
            characterIndex += step
            if (isUtf16SurrogateAtIndex(string, jsIndex)) {
                jsIndex += step * 2
            } else {
                jsIndex += step
            }
        }
    }

    return -1
}

Object.assign(String.prototype, {
    $clone() {
        return new this.constructor(this.valueOf())
    },

    size() {
        return this.length
    },

    getCharCode(n) {
        let jsIndex = jsIndexForNthCharacter(this, n)
        if (jsIndex === -1) {
            return ["none"]
        } else {
            return ["some", this.codePointAt(jsIndex)]
        }
    },
})
