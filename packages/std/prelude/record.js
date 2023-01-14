Object.assign(Object.prototype, {
    $clone() {
        return new this.constructor({ ...this.valueOf() })
    },

    /** Allows us to set a key on an object without mutating the original object.
    /*
    /* In Buri, you can write the following code:
    /*
    /* ```buri
    /* person = { name: "Sam", age: 30 }
    /* theodore = person:set(#name, "Theodore")
    /* ```
    /*
    /* This is compiled to the following JS code:
    /*
    /* ```js
    /* let person = { name: "Sam", age: 30 }
    /* let theodore = person.$set("name", "Theodore")
    /* ```
     */
    set(key, value) {
        return new this.constructor({ ...this.valueOf(), [key]: value })
    },
})
