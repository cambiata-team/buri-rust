export const getTagName = (tag) => {
    return tag[0]
}

export const getTagContents = (tag) => {
    let contents = [...tag]
    contents.shift()
    return contents
}

export const tag = (name, ...contents) => {
    if (name == "true" && contents.length == 0) return true
    if (name == "false" && contents.length == 0) return false
    return [name, ...contents]
}

export const getEnumValue = (buriEnum) => {
    return typeof buriEnum === "number" ? buriEnum : buriEnum[0]
}

export const getEnumContents = (buriEnum) => {
    let contents = [...buriEnum]
    contents.shift()
    return contents
}
