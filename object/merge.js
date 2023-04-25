module.exports = (...objects) => {
    let a = {}
    objects.forEach(x => {if (!Array.isArray(x) && (typeof x == "object")) a = {...a, ...x}})
    return a
}