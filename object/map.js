module.exports = (obj, func) => {
    let a = {}
    Object.keys(obj).forEach(i => a[i] = func(i, obj[i]))
    return a
}  