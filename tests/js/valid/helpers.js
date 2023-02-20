export const getTagName = (tag) => {
    return tag[0]
}

export const getTagContents = (tag) => {
    let contents = [...tag]
    contents.shift()
    return contents
}
