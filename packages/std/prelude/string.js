Object.assign(String.prototype, {
    $clone() {
        return new this.constructor(this.valueOf())
    },

    size() {
        return this.length
    },

    /// Assume that the string is valid UTF-16.
    /// Return the offset of the nth character in the string.
    /// If n is negative, return the offset of the character that is nth from the end.
    /// If the nth character is composed of a surrogate pair, return the offset of the high surrogate.
    /// If n is out of bounds, return -1.
    jsIndexForNthCharacter(n) {
        // ((this.charCodeAt(jsIndex) & 0xF800) === 0xD800) is true if and only if the code point at jsIndex is a UTF-16 surrogate.
        if (n >= 0) {
            if (n >= this.length) {
                return -1
            }
            let characterIndex = 0;
            let jsIndex = 0;
            while (true) {
                if (jsIndex >= this.length) {
                    return -1
                }
                else if (characterIndex === n) {
                    return jsIndex
                }
                else {
                    characterIndex += 1
                    jsIndex += 1 + ((this.charCodeAt(jsIndex) & 0xF800) === 0xD800)
                }
            }
        }
        else {
            if (n < -this.length) {
                return -1
            }
            let characterIndex = -1
            let jsIndex = this.length - 1
            while (true) {
                if (jsIndex < 0) {
                    return -1
                }
                else if (characterIndex === n) {
                    return jsIndex - ((this.charCodeAt(jsIndex) & 0xF800) === 0xD800)
                }
                else {
                    characterIndex -= 1
                    jsIndex -= 1 + ((this.charCodeAt(jsIndex) & 0xF800) === 0xD800)
                }
            }
        }
    },

    getCharCode(n) {
        let jsIndex = this.jsIndexForNthCharacter(n)
        if (jsIndex === -1) {
            return ["none"]
        }
        else {
            return ["some", this.codePointAt(jsIndex)]
        }
    },
})
